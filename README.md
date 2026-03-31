<p align="center">
  <img src="https://github.com/user-attachments/assets/0433af62-a837-4b78-bcc3-71445c7de5cd" alt="RavenDoor" Logo" />
</p>

<p align="center">
  <img width="300" height="450" alt="RavenRAT" src="https://github.com/user-attachments/assets/ed8e2af1-83ba-43b2-b31e-1504e910af84" />
</p>


<p align="center">
   <a href="https://dotnet.microsoft.com/">
    <img src="https://img.shields.io/badge/Rust-Backdoor-B70000.svg">
  </a>
    <img src="https://img.shields.io/badge/System-Windows-B70000.svg">
  </a>
    <img src="https://img.shields.io/badge/Version-2.0-B70000.svg">
  </a>
    <img src="https://img.shields.io/badge/Private-%F0%9F%94%92-B70000.svg">
  </a>
</p>

<h1 align="center"></h1>

### Características `v1.0`:

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

<img width="489" height="258" alt="telegram" src="https://github.com/user-attachments/assets/da29a2be-55e7-4513-ac11-258b7ff766b8" />

<h1 align="center"></h1>

### Características `v2.0`:

<img src="https://img.shields.io/badge/DLL_INJECTION:-B70000.svg"> Extrae una DLL embebida en el ejecutable, la escribe en disco en una ubicación discreta y la inyecta en procesos legítimos del sistema (como `explorer.exe`, `notepad.exe`, etc). Prioriza el uso de indirect syscalls para las operaciones críticas, con fallback automático a WinAPI para garantizar compatibilidad. Una vez inyectada, la DLL establece su propia conexión con el C2, permitiendo mantener el control incluso si el proceso principal es detectado o finaliza.

<img width="492" height="303" alt="process_injection" src="https://github.com/user-attachments/assets/30fa929b-0568-46a3-9e0b-82bba0ca623b" />

<img width="1919" height="923" alt="process_injection" src="https://github.com/user-attachments/assets/c9264b4b-f7fb-4df1-8874-48bfaebbe083" />

<img width="1265" height="618" alt="process_injection" src="https://github.com/user-attachments/assets/1472afa7-7772-4132-bc99-f7bcecbd8bc1" />


<h1 align="center"></h1>

<img src="https://img.shields.io/badge/DROPPER:-B70000.svg"> Módulo auxiliar para gestionar artefactos en disco: extrae y oculta archivos embebidos, crea copias de DLLs legítimas para técnicas de sideloading, limpia rastros temporales y detecta entornos de análisis (sandbox) mediante identificación de procesos típicos (`vboxservice.exe`, `vmtoolsd.exe`) o características de hardware, permitiendo abortar la ejecución en contextos no deseados.

<h1 align="center"></h1>

<img src="https://img.shields.io/badge/SLEEP OBFUSCATION:-B70000.svg"> Implementa una técnica de ofuscación temporal que evita patrones de espera predecibles. En lugar de usar `Sleep()`, emplea `NtDelayExecution`, una llamada directa al kernel que reduce la dependencia de APIs más monitoreadas. Además, introduce **jitter dinámico**, haciendo que cada pausa tenga una duración variable dentro de un rango configurable, dificultando la detección basada en comportamiento. Durante los períodos de inactividad, el módulo puede proteger regiones críticas de memoria cambiando sus permisos (`PAGE_NOACCESS`) y cifrando su contenido temporalmente, restaurando ambos al despertar. Esto reduce la superficie de ataque y minimiza la exposición de datos sensibles mientras el proceso no está activo.

<h1 align="center"></h1>

<img src="https://img.shields.io/badge/INDIRECT SYSCALLS:-B70000.svg"> Implementa un método de ejecución de llamadas al sistema que evita las rutas tradicionales de las APIs de Windows, reduciendo la visibilidad ante soluciones de seguridad. En lugar de usar funciones como `GetProcAddress`, localiza manualmente `ntdll.dll` recorriendo la estructura PEB del proceso, analiza su tabla de exportaciones para obtener las direcciones de funciones clave y extrae los números de syscall directamente desde el código de estas. Luego, en lugar de ejecutar la syscall de forma directa, redirige el flujo de ejecución hacia la instrucción `syscall` dentro de `ntdll.dll`, haciendo que el origen de la llamada parezca legítimo. Esto evita los hooks en capas de usuario, no deja rastros de imports estáticos y dificulta la detección por parte de sistemas de monitoreo, permitiendo que las operaciones críticas pasen desapercibidas.

<h1 align="center"></h1>

<img src="https://img.shields.io/badge/TOKEN_GENERATOR:-B70000.svg"> Incluye una utilidad en Python que ofusca automáticamente las credenciales del bot de Telegram. El script toma el token y el chat ID como entrada y genera código Rust con múltiples capas de ofuscación: codificación hexadecimal, XOR con clave fija, desplazamiento de caracteres o fragmentación. Esto evita que las credenciales aparezcan en texto plano dentro del binario final, dificultando el análisis estático.

<img width="1097" height="290" alt="token-generate" src="https://github.com/user-attachments/assets/15dc2584-e0b8-4884-8036-0cd35f3a50d0" />

<h1 align="center"></h1>

<img src="https://img.shields.io/badge/VPS SERVER:-B70000.svg"> Tiene contratado un servicio VPS seguro con ubicación en un país conflictivo. El hecho de que se utilice un VPS localizado en un país con este tipo de contexto puede proporcionarle cierta cobertura jurídica y operativa, ya que las agencias de inteligencia pueden enfrentar mayores dificultades para requerir datos y coordinar acciones, lo que introduce demoras y fricción en el proceso investigativo. No obstante, esto no imposibilita el rastreo, sino que, en todo caso, puede dificultar el trabajo de las agencias y alargar los tiempos necesarios para avanzar en la investigación.

<img width="1082" height="642" alt="VPS-Server" src="https://github.com/user-attachments/assets/5aed183d-d14e-4755-b73f-739d1fbe7f3c" />

<h1 align="center"></h1>

<img src="https://img.shields.io/badge/ANTI SANDBOX:-B70000.svg"> Implementa un sistema de detección de entornos de análisis basado en puntuación que combina múltiples indicadores del sistema: procesos relacionados a virtualización o monitoreo, presencia de drivers y artefactos característicos, claves de registro, información de hardware y comportamiento de recursos (memoria, CPU, disco y resolución). También incluye detección de debuggers y herramientas de sandbox.

Cada señal suma a un puntaje global; al superar un umbral predefinido, el módulo asume que se encuentra en un entorno controlado y aborta la ejecución de forma silenciosa. Este enfoque evita depender de un único indicador y reduce falsos positivos en entornos legítimos.

<h1 align="center"></h1>

Correo de contacto:

<img src="https://img.shields.io/badge/r3li4nt.contact@keemail.me-B70000?style=for-the-badge&logo=gmail&logoColor=white" />

<h1 align="center"></h1>

#### Developer: ~R3LI4NT~
