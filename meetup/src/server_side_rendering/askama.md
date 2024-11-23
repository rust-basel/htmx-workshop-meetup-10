# Askama

Askama is a popular Rust templating engine inspired by Jinja2 and Twig.
It focuses on simplicity, safety, and efficiency by compiling templates into pure Rust code at build time.
In this chapter, we will explore what makes Askama unique and learn how to use it effectively.

```html
<head>
    <title>{{ title | upper }}</title>
</head>
<body>
    <h1>Hello, {{ user.name }}!</h1>
    
    {% if user.is_admin %}
        <p>Welcome, Admin!</p>
    {% else %}
        <p>Welcome, User!</p>
    {% endif %}
    
    <ul>
        {% for role in user.roles %}
            <li>{{ role }}</li>
        {% endfor %}
    </ul>
</body>
</html>
```
<center>example askama snippet displaying filter, conditional rendering and a loop</center>

## Template Engines
Template engines are tools used to generate dynamic content (mostly, but not exclusively HTML).
They generate the content by combining static templates with runtime data.
Historically, they evolved out of server side includes and PHP, 
where embedding logic directly in HTML became a standard practice for creating dynamic web pages. 
Over time, modern template engines emerged with cleaner syntax, 
better safety features, and more structured approaches to templating.
They provide various properties that contribute to their usability and functionality:

### Safety Through Auto-Escaping
Modern template engines like Askama ensure safety by escaping special characters in user input. 
This prevents common security issues like Cross-Site Scripting (XSS) in web applications. 
By default, Askama escapes potentially unsafe characters when rendering HTML templates.
```rust
let user = User {
    name: "Alice",
    bio: "<script>alert('Hacked!')</script>",
};
```
Scripts are by default escaped so they are not executed. `{{ bio }}` will be rendered like this:
```
&lt;script&gt;alert('Hacked!')&lt;/script&gt;
```

### Minimal Set of Control Syntax
Template Engines provide a concise and minimal syntax for common control structures like *loops*, *conditionals*, and *filters*.
This reduces boilerplate and improves template readability.

### Simplicity and Composability
Templates are meant to be simple and composable. 
Template Engines typically support features like *includes* and *inheritance*, 
which make it easy to break large templates into smaller, reusable pieces.

## Properties of Askama

### Compiled Templates
Askama compiles templates into Rust code at build time.
This approach ensures type safety, better performance, and early detection of syntax errors. 
Unlike interpreted engines like Tera, Askama doesn’t evaluate templates at runtime, 
making it faster and less error-prone at runtime.
Tera, on the other hand, guarantees a more rapid development cycle especially for more complex projects, 
exactly because it can get around the compilation step.

### Inspiration from Jinja2 and Twig
Askama’s syntax is heavily inspired by Jinja2 (Python) and Twig (PHP).
Developers familiar with these engines will find Askama’s syntax intuitive and easy to adopt.
The same is true for Tera.

| **Feature**               | **Askama**                                                              | **Jinja2**                                                             |
|---------------------------|-------------------------------------------------------------------------|------------------------------------------------------------------------|
| **Variable interpolation**| `Hello, {{ name }}!`                                                    | `Hello, {{ name }}!`                                                   |
| **Conditionals**          | `{% if is_logged_in %}Welcome!{% else %}Please log in.{% endif %}`      | `{% if is_logged_in %}Welcome!{% else %}Please log in.{% endif %}`     |
| **Loops**                 | `{% for item in items %}<li>{{ item }}</li>{% endfor %}`                | `{% for item in items %}<li>{{ item }}</li>{% endfor %}`               |
| **Includes**              | `{% include "header.html" %}`                                           | `{% include "header.html" %}`                                          |
| **Template inheritance**  | `{% extends "base.html" %}{% block content %}Hello{% endblock %}`       | `{% extends "base.html" %}{% block content %}Hello{% endblock %}`      |
| **Filters**               | `{{ name \| upper }}`                                                   | `{{ name \| upper }}`                                                  |
| **Macros**                | `{% macro greet(name) %}Hello, {{ name }}!{% endmacro %}`               | `{% macro greet(name) %}Hello, {{ name }}!{% endmacro %}`              |
| **Match-like structures** | `{% match status %}{% when "success" %}OK{% else %}Error{% endmatch %}` | *Not available directly in Jinja2 (requires custom filters or logic)*  |
| **Rust-specific syntax**  | `{% if let Some(value) = optional %}{{ value }}{% endif %}`             | *Not available directly in Jinja2 (requires pre-processing in Python)* |

## How-to
This section provides a step-by-step guide to using Askama in your projects.

### Include Askama in Your App
Add Askama to your `Cargo.toml` file:
```toml
[dependencies]
askama = "0.12"
```

### Locate Templates in Your project
Templates are stored in a templates directory by convention. 
File extensions like `.html` or `.txt` indicate the template's format.

For example, `templates/hello.html`:
```html
<!DOCTYPE html>
<html>
<head>
    <title>Hello, {{ name }}!</title>
</head>
<body>
    <h1>Welcome, {{ name }}!</h1>
</body>
</html>
```

### Use Variables / Render Templates
Pass data to templates using Rust structs:
```rust
use askama::Template;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

fn main() {
    let hello = HelloTemplate { name: "Alice" };
    println!("{}", hello.render().unwrap());
}
```

### Architect with Template Inheritance / Blocks
Template inheritance allows defining base layouts and extending them with specific blocks.
For example:
```html
<!-- templates/base.html -->
<!DOCTYPE html>
<html>
<head>
    <title>{% block title %}Default Title{% endblock %}</title>
</head>
<body>
    {% block content %}{% endblock %}
</body>
</html>
```
```html
<!-- templates/home.html -->
{% extends "base.html" %}

{% block title %}Home{% endblock %}

{% block content %}
<h1>Welcome to the home page!</h1>
{% endblock %}
```

### Compose with Includes
Use `{% include %}` to embed reusable components:
```html
<!-- templates/base.html -->
<!DOCTYPE html>
<html>
<head>
    <title>{% block title %}Default Title{% endblock %}</title>
</head>
<body>
    {% include "header.html" %}
    {% block content %}{% endblock %}
    {% include "footer.html" %}
</body>
</html>
```

### Use Loops and Conditionals
Render dynamic content using loops and conditionals:
```html
<ul>
{% for item in items %}
    <li>{{ item }}</li>
{% endfor %}
</ul>

{% if condition %}
    <p>Condition is true!</p>
{% else %}
    <p>Condition is false!</p>
{% endif %}
```
Rust-Specific if let and match
Askama supports Rust's `if let` and `match` expressions for more complex logic:
```html
{% if let Some(value) = optional %}
    <p>Value: {{ value }}</p>
{% else %}
    <p>No value found.</p>
{% endif %}

{% match status %}
    {% when "success" %}<p>Success!</p>{% endmatch %}
    {% when "error" %}<p>Error occurred.</p>{% endmatch %}
    {% else %}<p>Unknown status.</p>{% endmatch %}
```

### Use Filters
Filters process variables for display:
```html
<p>{{ message|upper }}</p>
```
*Note that the pipe symbol must not be surrounded by spaces; otherwise, it will be interpreted as the BitOr operator.*

### Register Your Own Filters
Custom filters can be registered in Rust code:
```rust
mod filters {
    pub fn reverse(s: &str) -> askama::Result<String> {
        Ok(s.chars().rev().collect())
    }
}
```

## Full Example: Integrating Askama with Axum
### Step 1: Template Definition
Create an Askama template file, `templates/hello.html`:

```html
<!DOCTYPE html>
<html>
<head>
    <title>{{ title }}</title>
</head>
<body>
    <h1>Hello, {{ user.name }}!</h1>
</body>
</html>
```
Define the corresponding Rust struct for the template:

```rust
use askama::Template;

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloTemplate<'a> {
    pub title: &'a str,
    pub user: User<'a>,
}

pub struct User<'a> {
    pub name: &'a str,
}
```
### Step 2: Axum Handler with impl IntoResponse
Set up an Axum handler that renders the Askama template:

```rust
use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use askama::Template; // Bring in the Template trait for rendering.
use crate::{HelloTemplate, User};

async fn hello_handler() -> impl IntoResponse {
    let user = User { name: "Alice" };
    let template = HelloTemplate {
        title: "Hello Page",
        user,
    };

    match template.render() {
        Ok(html) => Response::builder()
            .header("Content-Type", "text/html")
            .body(html)
            .unwrap(),
        Err(err) => {
            eprintln!("Template rendering error: {}", err);
            Response::builder()
                .status(500)
                .body("Internal Server Error".into())
                .unwrap()
        }
    }
}
```
### Step 3: Set Up Axum Application
Create an Axum app and route the handler:

```rust
#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_handler));
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```