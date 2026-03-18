Portfolio project inspired by https://github.com/cfereday

# Mauro's Portfolio Website

A personal portfolio website built with Rust

deployes on render available here: https://portfolio-kvnv.onrender.com

## ✨ Features

<!-- - **Blog System**: Markdown-powered blog with status tags (draft/complete) -->
- **Experience Timeline**: Professional background showcase
- **Responsive Design**: Mobile-friendly layout
- **Fast Performance**: Rust server

## 🛠 Tech Stack

- **Backend**: Rust with html/template
- **Styling**: CSS with Grid and Flexbox

## 📁 Project Structure

```
Portfolio/
├── content/
│   └── blog/          # Markdown blog posts
├── static/
│   ├── css/           # Stylesheets
│   └── assets/        # Static images, CV
├── templates/         # HTML templates
├── parser/            # Markdown parsing logic for blogs
│    src/
│   └── main.rs        # Rust server
├── Cargo.toml         # Rust dependencies
```

### Setup

3. **Run the server**
   ```bash
   make dev
   ```

4. **Visit the site**
   Open [http://localhost:8080](http://localhost:8080)

## 📝 Adding Blog Posts

Create a new markdown file in `content/blog/` with the following format:

```markdown
# Your Post Title

*Published: July 1, 2025*
*Category: Technology*
*Status: draft*

Your blog post content here...
```

**Supported fields:**
- `Published`: Date in "Month Day, Year" format
- `Category`: Post category
- `Status`: `draft` or `complete` (shows colored tag)