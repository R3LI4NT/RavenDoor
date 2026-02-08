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

<img src="https://img.shields.io/badge/ENCRIPTACIÓN:-B70000.svg"> Incluye un loader que carga y ejecuta los módulos (`EncryptModule.dll`, `SystemDiscovery.dll`, `ProcessKiller.dll`, `RClone.dll`, `Remover.dll`, `EDRKiller.dll`, `SMBSpreader.dll`, `Backdoor[svchost.dll]`)  directamente desde memoria, evitando que el archivo exista en el disco. Esta técnica reduce la superficie de detección y dificulta la ingeniería inversa. Para reforzar la protección, los archivos .DLL son ofuscados empleando múltiples técnicas (**Strings Encryption, Control Flow Obfuscation, Resource Encryption, Dead Code Injection, Metadata Pruning, Linking, PreMark, Anti-Debug, etc**).
