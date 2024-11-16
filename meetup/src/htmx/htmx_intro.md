# Why htmx?

## You are doing REST wrong

>I am getting frustrated by the number of people calling any HTTP-based interface a REST API.
>Today’s example is the SocialSite REST API. That is RPC.
>It screams RPC.
>There is so much coupling on display that it should be given an X rating.
>What needs to be done to make the REST architectural style clear on the notion that hypertext is a constraint? In other words, if the engine of application state (and hence the API) is not being driven by hypertext, then it cannot be RESTful and cannot be a REST API.
>Period.
>Is there some broken manual somewhere that needs to be fixed?

–Roy Fielding, Creator of the term REST

## Users do not understand Json

No, but the browser can convert: 

```html
<button hx-post="/clicked"
    hx-trigger="click"
    hx-target="#parent-div"
    hx-swap="outerHTML"
>
    Click Me!
</button>
```

to a Button

## Attributes over javascript

With javascript:

```html
<button id="my-button" on-click="clicked()">
    Click Me!
</button>
<script>
function clicked(){
  const request = new XMLHttpRequest();

  xhr.onreadystatechange = () => {
    if (xhr.readyState === 4) {
       const element = document.getElementById("parent-div");
       element.outerHtml = xhr.response
    }
  };

  request.open('POST', "/clicked");
  request.send(null);
}

</script>
```

With Htmx:
```html
<button hx-post="/clicked"
    hx-trigger="click"
    hx-target="#parent-div"
    hx-swap="outerHTML"
>
    Click Me!
</button>
```

This was a simple example, but we already see how concise Htmx can be.

Now infinite scroll:

With javascript:

```html
<tr 
  <td>Agent Smith</td>
  <td>void29@null.org</td>
  <td>55F49448C0</td>
</tr>

<script>
//... yeah not going to do that
</script>
```


With Htmx:

```html
<tr hx-get="/contacts/?page=2"
    hx-trigger="revealed"
    hx-swap="afterend">
  <td>Agent Smith</td>
  <td>void29@null.org</td>
  <td>55F49448C0</td>
</tr>
```


