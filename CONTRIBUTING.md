# Contribuir a Kitsune Guardian Fox

¡Gracias por tu interés en contribuir a Kitsune! 🦊

## 🚀 Cómo Contribuir

### Reportar Bugs

Si encuentras un bug, por favor abre un [issue](https://github.com/aguitauwu/Kitsune/issues) con:
- Descripción clara del problema
- Pasos para reproducirlo
- Comportamiento esperado vs actual
- Versión de Rust, SO, y dependencias
- Logs relevantes (sin información sensible)

### Sugerir Mejoras

Para sugerencias de nuevas características:
1. Abre un issue describiendo la funcionalidad
2. Explica por qué sería útil
3. Proporciona ejemplos de uso

### Pull Requests

1. **Fork el repositorio**
2. **Crea una rama**: `git checkout -b feature/mi-feature`
3. **Sigue las guías de estilo de código**
4. **Escribe tests** si aplica
5. **Actualiza documentación** si es necesario
6. **Commit con mensajes descriptivos**
7. **Push**: `git push origin feature/mi-feature`
8. **Abre un Pull Request**

## 📝 Guías de Estilo

### Código Rust
- Usa `cargo fmt` antes de commit
- Ejecuta `cargo clippy` y corrige warnings
- Sigue las convenciones de Rust
- Documenta funciones públicas con `///`
- Escribe código idiomático y eficiente

### Mensajes de Commit
```
tipo: descripción breve

Descripción más detallada si es necesario.

Closes #123
```

Tipos: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

### Documentación
- Mantén el README.md actualizado
- Documenta nuevas configuraciones
- Incluye ejemplos de uso

## 🧪 Testing

```bash
# Ejecuta todos los tests
cargo test

# Tests con output
cargo test -- --nocapture

# Test específico
cargo test nombre_del_test
```

## 🔍 Checklist para PR

- [ ] El código compila sin errores
- [ ] Pasa `cargo fmt`
- [ ] Pasa `cargo clippy`
- [ ] Tests incluidos (si aplica)
- [ ] Documentación actualizada
- [ ] Commit messages claros
- [ ] Sin secretos o información sensible

## 📞 Preguntas

¿Dudas? Abre un issue con la etiqueta `question` o únete a nuestro Discord.

¡Gracias por contribuir! 🎉
