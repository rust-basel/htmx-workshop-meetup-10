# HTMX samples


### Simple Button

```html
    <h1>HTMX Example</h1>
    <button hx-get="/message" hx-target="#message">Click me!</button>
    <div id="message"></div>
```    

### Form

```html
 <h1>HTMX Example</h1>
    <form
    hx-post="/message" 
    hx-target="#message"
    >
        
    <input type="text" name="message" placeholder="Enter your message">

    <button type="submit" >Click me!</button>
    </form>
    
    <div id="message"></div>
```

### Composition

```html
<h1>Example</h1>
<div hx-get="/component1" hx-trigger="load delay:2s">
   
</div>
```

```html
<h1>HTMX Component</h1>
<div>
    
   <p>Component 1</p>
   
   <button hx-get="/message"  hx-target="#message" >
       Get message
   </button>

<div id="message"></div>

</div>
```