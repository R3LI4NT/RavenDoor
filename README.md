<p align="center">
  <img src="https://github.com/user-attachments/assets/0433af62-a837-4b78-bcc3-71445c7de5cd" alt="RavenDoor" Logo" />
</p>

<p align="center">
  <img width="250" height="400" alt="RavenRAT" src="https://github.com/user-attachments/assets/236c245b-8812-4721-a428-7a7e5119773b" />
</p>


<p align="center">
   <a href="https://dotnet.microsoft.com/">
    <img src="https://img.shields.io/badge/Rust-Backdoor-B70000.svg">
  </a>
    <img src="https://img.shields.io/badge/System-Windows-B70000.svg">
  </a>
    <img src="https://img.shields.io/badge/Public-%F0%9F%97%9D%EF%B8%8F-B70000.svg">
  </a>
</p>

<h1 align="center"></h1>

### Características de `RavenDoor`:

<img src="https://img.shields.io/badge/COMUNICACIÓN:-B70000.svg"> Genera una llave (AES-256-CBC) **ÚNICA** para cada usuario infectado. Con este cifrado, los firewalls, sistemas IDS/IPS o herramientas de monitoreo de red no podrán detectar los comandos en texto plano que envía el atacante, evitando activar alertas. Se implemento el algoritmo AES-256 para transformar estos comandos reconocibles en flujos de bytes aleatorios, mientras que el CBC asegura que incluso comandos idénticos produzcan salidas cifradas diferentes, eliminando patrones repetitivos que podrían servir como firmas de detección.

<img width="1095" height="425" alt="2" src="https://github.com/user-attachments/assets/67130bbe-6acf-47c7-80a3-53f149127eb7" />

<h1 align="center"></h1>

<img src="https://img.shields.io/badge/PERSISTENCIA:-B70000.svg"> Implementa distintos mecanismos de persistencia en Windows para que el ejecutable se vuelva a iniciar automáticamente tras reinicios o cierres de sesión. A través del método indicada (registry, startup, scheduled o service), el programa obtiene la ruta de su propio ejecutable y la registra para ejecutarse al inicio del sistema: puede crear una clave en el registro de usuario (Run), copiarse en la carpeta Startup, crear una tarea programada al iniciar sesión (intentando ejecutarse como SYSTEM) o instalarse como servicio de Windows con inicio automático. Si un método falla, el código intenta otros alternativos como respaldo.

<h1 align="center"></h1>

<img src="https://img.shields.io/badge/C2:-B70000.svg"> Se desarrollo un C2 (Command & Control) en Python para escuchar las conexiones entrantes, gestiona múltiples clientes, permite seleccionar un cliente y enviarle comandos interactivos. La comunicación esta cifrada con AES-256 en modo CBC, usando una clave compartida pasada por parámetro y un IV que el cliente envía en el handshake. 

**USO:**
```bash
python3 raven_client.py --key [KEY]
```

<img width="1113" height="682" alt="1" src="https://github.com/user-attachments/assets/dc240f51-a21a-4bac-adbf-1998ef5dc11e" />

El archivo `ravendoor_key.txt` es generado en la ruta `AppData\Roaming\Microsoft\Windows` y es envíado al canal de telegram para su posterior uso.

<img width="510" height="214" alt="3" src="https://github.com/user-attachments/assets/6bedc1d2-39af-4910-813b-c7ead251b0d2" />

<h1 align="center"></h1>

### Modo de uso

- (1) Tener instalado Rust: https://rust-lang.org/tools/install/

- (2) Modificar los archivos `config.toml`, `main.rs` y `config.rs` para agregar nuestras IPs y puerto a escuchar.

<img width="351" height="118" alt="4-0" src="https://github.com/user-attachments/assets/c5ac7947-1a06-495c-ba76-eb5d3a32e04e" />

<img width="523" height="156" alt="4-1" src="https://github.com/user-attachments/assets/9cedc726-d38f-475e-953c-c146ae731248" />

<img width="584" height="189" alt="4-2" src="https://github.com/user-attachments/assets/33b16a54-a3a6-4509-9fde-9094557c649c" />

- (3) Generar un Bot en Telegram y agregar el Token y ChatID en el archivo telegram.rs.

<img width="408" height="138" alt="5" src="https://github.com/user-attachments/assets/9d97ea5c-8d85-4ee3-bdaa-a79a8f88a98c" />

**IMPORTANTE:** La función `create_bot_from_env` no debe ser modificada por el Token ni ChatID.

- (4) Compilar proyecto en modo release desde la raíz.
```
cargo build --release
```

<h1 align="center"></h1>

Correo de contacto:

<img src="https://img.shields.io/badge/r3li4nt.contact@keemail.me-D14836?style=for-the-badge&logo=gmail&logoColor=white" />

<h1 align="center"></h1>

> [!CAUTION]
> Cualquier uso indebido de este software será de exclusiva responsabilidad del usuario final, y no del autor. Este proyecto tiene como objetivo inicial demostrar las capacidades de Rust como lenguaje para el desarrollo de malware en entornos controlados. 

<h1 align="center"></h1>

#### Developer: ~R3LI4NT~
