# 🦊 Kitsune Guardian Fox

<div align="center">
  <img src="assets/kitsune.jpg" alt="Kitsune Guardian Fox" width="400"/>
  
  **Advanced Discord Security Bot built with Rust 🦀**

  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
  [![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
  [![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?style=flat&logo=discord&logoColor=white)](https://discord.com/)
  
  ![GitHub Stars](https://img.shields.io/github/stars/aguitauwu/Kitsune?style=social)
  ![GitHub Forks](https://img.shields.io/github/forks/aguitauwu/Kitsune?style=social)
  ![GitHub Issues](https://img.shields.io/github/issues/aguitauwu/Kitsune)
  ![GitHub last commit](https://img.shields.io/github/last-commit/aguitauwu/Kitsune)
</div>

## 📖 Descripción

Kitsune Guardian Fox es un bot de seguridad avanzado para Discord diseñado para proteger tu servidor contra amenazas, raids, spam y comportamiento malicioso. Construido con Rust para máximo rendimiento y confiabilidad.

## ✨ Características Principales

### 🛡️ Seguridad
- **Detección de Raids en Tiempo Real**: Identifica y mitiga raids automáticamente
- **Análisis de Comportamiento**: Sistema de IA para detectar patrones sospechosos
- **Sistema Honeypot**: Trampas para identificar bots y usuarios maliciosos
- **Auto-Moderación**: Acciones automáticas basadas en niveles de amenaza

### 📊 Análisis y Estadísticas
- **Análisis ML**: Machine learning para predicción de amenazas
- **Métricas de Servidor**: Estadísticas detalladas de actividad y seguridad
- **Informes Forenses**: Investigación detallada de incidentes
- **Tablas de Clasificación**: Ranking de usuarios y actividad

### 👥 Gestión de Usuarios
- **Sistema de Reputación**: Red de reputación entre servidores
- **Whitelist/Blacklist**: Control granular de acceso
- **Sincronización**: Comparte listas entre servidores aliados
- **Verificación**: Sistema de verificación de nuevos usuarios

### ⚙️ Moderación
- **Comandos Completos**: Ban, kick, timeout, warn, pardon
- **Lockdown**: Cierre de emergencia del servidor
- **Configuración Flexible**: Ajusta umbrales y comportamiento
- **Notificaciones**: Alertas en tiempo real para moderadores

### 💾 Utilidades
- **Backup Automático**: Copia de seguridad de configuración
- **Webhooks**: Integración con sistemas externos
- **Analytics Avanzado**: Predicciones y comparaciones
- **Comandos Personalizados**: Crea respuestas y acciones personalizadas

## 🚀 Instalación

### Requisitos Previos
- Rust 1.70 o superior
- PostgreSQL 14 o superior
- Redis 6.0 o superior
- Token de Discord Bot

### Configuración

1. **Clona el repositorio**
```bash
git clone https://github.com/aguitauwu/Kitsune.git
cd kitsune
```

2. **Crea el archivo de configuración**
```bash
cp config.example.toml config.toml
```

3. **Configura las variables de entorno**
```bash
# .env
DATABASE_URL=postgres://usuario:contraseña@localhost/kitsune
REDIS_URL=redis://localhost:6379
DISCORD_TOKEN=tu_token_aqui
```

4. **Edita `config.toml`** con tus preferencias de seguridad

5. **Compila y ejecuta**
```bash
cargo build --release
./target/release/kitsune
```

## 📋 Comandos Disponibles

### Comandos Principales

| Comando | Descripción |
|---------|-------------|
| `/kitsune` | Información principal del bot |
| `/help` | Muestra todos los comandos disponibles |
| `/info` | Información sobre categorías de comandos |

### Seguridad
- `/kitsune status` - Estado de seguridad del servidor
- `/kitsune scan` - Escaneo completo del servidor
- `/kitsune check @usuario` - Analiza un usuario específico
- `/kitsune analyze` - Análisis de amenazas

### Moderación
- `/kitsune ban @usuario [razón]` - Banea un usuario
- `/kitsune kick @usuario [razón]` - Expulsa un usuario
- `/kitsune timeout @usuario [duración]` - Timeout temporal
- `/kitsune warn @usuario [razón]` - Advierte a un usuario
- `/kitsune unban ID [razón]` - Remueve ban
- `/kitsune pardon @usuario` - Perdona advertencias

### Configuración
- `/kitsune view` - Ver configuración actual
- `/kitsune automod_toggle` - Activar/desactivar auto-mod
- `/kitsune channel [canal]` - Configurar canales
- `/kitsune notify` - Configurar notificaciones
- `/kitsune raid` - Configurar detección de raids
- `/kitsune behavior` - Configurar análisis de comportamiento
- `/kitsune ml` - Configurar machine learning

### Listas de Acceso
- `/access whitelist @usuario [razón]` - Agregar a whitelist
- `/access whitelist_remove @usuario` - Remover de whitelist
- `/access whitelist_list` - Ver whitelist
- `/access blacklist ID [razón]` - Agregar a blacklist
- `/access blacklist_remove ID` - Remover de blacklist
- `/access blacklist_list` - Ver blacklist
- `/access blacklist_import [servidor]` - Importar blacklist
- `/access blacklist_export` - Exportar blacklist

### Estadísticas
- `/kitsune stats` - Estadísticas del servidor
- `/kitsune leaderboard` - Tabla de clasificación
- `/kitsune report` - Generar reporte
- `/kitsune forensics` - Análisis forense
- `/kitsune export` - Exportar datos

### Reputación
- `/reputation query @usuario` - Consultar reputación
- `/reputation report @usuario [razón]` - Reportar usuario
- `/reputation trust [servidor]` - Agregar servidor confiable
- `/reputation sync` - Sincronizar datos
- `/reputation servers` - Ver servidores en red
- `/reputation appeal [caso]` - Apelar reporte

### Utilidades Admin
- `/admin backup` - Crear backup
- `/admin webhook add [url]` - Agregar webhook
- `/admin test` - Probar funcionalidad
- `/admin custom` - Comandos personalizados

## 🏗️ Arquitectura

```
kitsune/
├── src/
│   ├── bot/              # Lógica del bot de Discord
│   │   ├── commands/     # Implementación de comandos
│   │   ├── events/       # Manejadores de eventos
│   │   └── mod.rs
│   ├── security/         # Sistemas de seguridad
│   │   ├── raid_detector.rs
│   │   ├── behavior_analyzer.rs
│   │   ├── honeypot.rs
│   │   └── auto_mod.rs
│   ├── database/         # Capa de base de datos
│   │   ├── models.rs
│   │   └── queries.rs
│   ├── config/           # Configuración
│   └── main.rs
├── config.toml           # Configuración del bot
├── Cargo.toml            # Dependencias Rust
└── README.md
```

## 🔧 Tecnologías

- **[Serenity](https://github.com/serenity-rs/serenity)** - Framework de Discord
- **[Poise](https://github.com/serenity-rs/poise)** - Framework de comandos
- **[SQLx](https://github.com/launchbadge/sqlx)** - Base de datos PostgreSQL
- **[Redis](https://redis.io/)** - Cache y estado temporal
- **[Tokio](https://tokio.rs/)** - Runtime asíncrono
- **[Tracing](https://github.com/tokio-rs/tracing)** - Logging y diagnósticos

## 📊 Base de Datos

Kitsune utiliza PostgreSQL para almacenar:
- Configuración de servidores
- Historial de moderación
- Datos de reputación
- Estadísticas y métricas
- Registros de incidentes

## 🤝 Contribuir

Las contribuciones son bienvenidas! Por favor:

1. Fork el proyecto
2. Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

## 📝 Licencia

Este proyecto está bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para más detalles.

Copyright (c) 2025 aguita

## 🙏 Agradecimientos

- Comunidad de Rust y Discord
- Todos los contribuidores del proyecto
- Librerías y frameworks utilizados

## 📞 Soporte

¿Necesitas ayuda? 
- Abre un [Issue](https://github.com/aguitauwu/Kitsune/issues)
- Lee la [Documentación](https://docs.kitsune.bot)
- Únete a nuestro [Discord de soporte](https://discord.gg/kitsune)

---

<div align="center">
  Hecho con ❤️ y 🦀 por aguita
</div>
