# 🚀 GitHub Setup Instructions

Este archivo contiene instrucciones para subir el proyecto a GitHub.

## 📦 Archivos Preparados

✅ **README.md** - Documentación principal con imagen
✅ **LICENSE** - Licencia MIT (2025 aguita)
✅ **.gitignore** - Archivos a ignorar
✅ **config.example.toml** - Ejemplo de configuración
✅ **.env.example** - Ejemplo de variables de entorno
✅ **CONTRIBUTING.md** - Guía de contribución
✅ **assets/kitsune.jpg** - Imagen del bot

## 🔧 Pasos para subir a GitHub

### 1. Crea un nuevo repositorio en GitHub
- Ve a https://github.com/new
- **Nombre**: `kitsune` (o el que prefieras)
- **Descripción**: "🦊 Advanced Discord Security Bot built with Rust"
- **Visibilidad**: Público o Privado según prefieras
- ⚠️ **NO inicialices** con README, .gitignore o LICENSE (ya los tenemos)

### 2. Conecta y sube el código

Ejecuta estos comandos en la terminal:

```bash
# Inicializa git (si no está inicializado)
git init

# Agrega todos los archivos
git add .

# Realiza el primer commit
git commit -m "feat: initial commit - Kitsune Guardian Fox Discord Bot"

# Renombra la rama a main
git branch -M main

# Conecta con GitHub (reemplaza TU_USUARIO con tu usuario de GitHub)
git remote add origin https://github.com/aguitauwu/Kitsune.git

# Sube el código
git push -u origin main
```

### 3. Verifica en GitHub

- Ve a tu repositorio en GitHub
- Verifica que todos los archivos estén presentes
- Asegúrate que la imagen se vea en el README

## 🔒 Archivos que NO se subirán

Los siguientes archivos están en `.gitignore` y NO se subirán (por seguridad):

- ❌ `config.toml` - Contiene configuración con secretos
- ❌ `.env` - Contiene tokens y contraseñas
- ❌ `/target` - Archivos compilados (muy pesados)
- ❌ `attached_assets/` - Archivos temporales

## 📋 Configuración Adicional (Opcional)

### Agregar Topics al Repositorio
1. Ve a tu repositorio en GitHub
2. Click en ⚙️ Settings
3. En "Topics" agrega:
   - `discord`
   - `discord-bot`
   - `rust`
   - `security`
   - `anti-raid`
   - `moderation`

### Configurar Secrets (para CI/CD)
1. Ve a **Settings** > **Secrets and variables** > **Actions**
2. Agrega estos secrets si planeas usar GitHub Actions:
   - `DISCORD_TOKEN`
   - `DATABASE_URL`
   - `REDIS_URL`

### Habilitar Discussions
1. Ve a **Settings** > **Features**
2. Marca la casilla **Discussions**

## 🎯 Próximos Pasos Sugeridos

- [ ] Configurar GitHub Actions para CI/CD
- [ ] Crear releases con tags semánticos (v1.0.0)
- [ ] Configurar Dependabot para actualizaciones
- [ ] Agregar Issue templates
- [ ] Crear Pull Request template
- [ ] Agregar Code of Conduct
- [ ] Configurar GitHub Pages para docs

## 📊 Badges Adicionales

Puedes agregar estos badges al README después de subir:

```markdown
![GitHub Stars](https://img.shields.io/github/stars/aguitauwu/Kitsune?style=social)
![GitHub Forks](https://img.shields.io/github/forks/aguitauwu/Kitsune?style=social)
![GitHub Issues](https://img.shields.io/github/issues/aguitauwu/Kitsune)
![GitHub Pull Requests](https://img.shields.io/github/issues-pr/aguitauwu/Kitsune)
```

## 🆘 Solución de Problemas

### Error: "remote origin already exists"
```bash
git remote remove origin
git remote add origin https://github.com/aguitauwu/Kitsune.git
```

### Error: "refusing to merge unrelated histories"
```bash
git pull origin main --allow-unrelated-histories
```

### Cambiar URL del repositorio remoto
```bash
git remote set-url origin https://github.com/TU_USUARIO/kitsune.git
```

---

¡Todo listo para GitHub! 🎉 Si tienes alguna duda, consulta la [documentación de GitHub](https://docs.github.com/).
