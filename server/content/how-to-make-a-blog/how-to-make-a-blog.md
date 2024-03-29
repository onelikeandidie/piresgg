# Recipe for a blog

If you wanted to make a blog like this one, you could use Jekyll, Hugo or Zola
to generate some static pages with a good theme and host them on GitHub pages
or Cloudflare. That would be the sane way to go about creating a blog, or an
even simpler way would be to use Medium, Tumblr or Blogger to create a blog
without needing to host the pages or even ever touch a piece of code ever.

Unfortunately, there is a parasite in my brain, that tells me to make
everything from scratch, and most of my projects suffer from that bug. I
might address this in a future post, but for now I will give you the recipe
for making this blog site.

## Ingredients

- A Markdown processor
- An http library
- A computer to host it on
- Disregard for development time
- Spare time
- Passion for making everything in Rust (even when you really don't need to)
- Inspiration

## Finding inspiration

The most important ingredient to a blog page for me is making it easy to read,
select, copy and browse through. If you look at Medium, a user-generated blog
content site mentioned earlier, they have a little thing that pops-up whenever
you highlight any piece of text with a shortcut to share that excerpt on X
(formerly Twitter). I think that it really distracts from the post content and
I didn't really want that.

Looking at blogger, it is very readable, but after browsing a few blogs, it
has very limited customization and I wanted to have a simpler look. I also
wanted all of my posts to be written not to a database but files in a git
repository so that I could back up every change I've ever made to the posts as
well as making the blog posts easy to edit with any text editor.

I eventually tried to find some other programmer's blogs, and the best way to
find them was to watch videos that "react" to blog posts, specifically the
youtubers "Theo" and "Primeagen", which actually led me to find the blog style
that I liked the most,
[Armin Ronacher's Thoughts and Writings](https://lucumr.pocoo.org/).

![The home page of Armin's blog](/public/images/how-to-make-a-blog-screenie-1.png)

![A blog post from Armin's blog](/public/images/how-to-make-a-blog-screenie-2.png)

Armin Ronacher seems to be a very prolific character in open-source communities
with a lot of contributions to several open-source libraries and applications
and their blog style is really what I want in a blog. So I had found my
inspiration.

## Picking the core flavour

Both at work and in my hobbies, I have created Markdown-based web projects and
applications, I really like Markdown. It is so simple to write even to non-web
developers. Here's a small excerpt of what markdown looks like:

```markdown
## Here is a heading, a title

This is a paragraph, _this test is in italics_, **this one is in bold**.

![This is an image](https://random.dog/2bff25d0-c721-4078-8cc9-f3ce6b464428.jpg)
```

Some of my projects already use markdown like
[Doky CMS PHP](https://github.com/onelikeandidie/doky-cms.php)
a laravel-based documentation serving site, which lets you write markdown
directly both in the git repository and the logged in frontend.

At work, I created a blog manager that lets you create more complicated blogs
for the [company's blog on Shopify](https://www.autofinesse.com/blogs/guides)
which I unfortunately cannot include screenshots of yet (but I wish I could).

Both of these CMS (content management systems) were made using Laravel and have
basis on markdown-to-html generation and were very fun to make since Laravel is
such a feature-full framework. But when I decided to make this blog, I decided
to move the starting point to another place, a place with fewer batteries
included and more from scratch programming.

## The spices to the blog

At this point I only had created one Rust-based website, the
[meetballs.org](https://meetballs.org/) site which is fully powered by Rust
libraries only (besides nginx).

![The home page of meetballs.org](/public/images/how-to-make-a-blog-screenie-3.png)

And I made that site after making the other two Laravel-based CMS sites but
wanted to explore more about creating a more dynamic site with Rust. I landed
on some starting libraries:

- [Actix-web](https://actix.rs/) - A simple web framework that just handles
    routing and handling multithreaded http requests and responses as well as
    application resources between threads. An example of a route is shown
    bellow.

```rust
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
```
- [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark/) - An
    efficient and reliable parser for CommonMark which is the standard of
    markdown I am used to writing in.
- [Tera](https://keats.github.io/tera/) - I was used to Twig, a couple of years
    ago at work we used to use OpenCart, which has Twig as a templating
    language. A base template for this post looks like this.

```twig
{% extends 'layouts/base' %}
{% block content %}
    <div class="container p-4 md:p-8">
        <article class="prose prose-neutral dark:prose-invert mx-auto">
            {{ content | safe }}
        </article>
    </div>
{% endblock content %}
```

- [TailwindCSS](https://tailwindcss.com/) - Styling library to help me speed up
    development time.

Connecting all these together to make a site was more difficult than I
initially expected. The fact that Actix was so bare bones meant that I had to
write all the application services that made rendering a template from Tera
possible, which means I ended up writing a wrapper that sits around Tera to
load all the templates files in and passing the render variables so that they
can be rendered easily on request. I also had to write a caching service so
that I could render the posts on request and save them for the next request.
Then I also had to write a loader service that would load the markdown files
at the start of the application into memory so that it could be faster to
render them once needed.

Another thing is that TailwindCSS is a npm package
([for now](https://tailwindcss.com/blog/tailwindcss-v4-alpha)) so I also had
to add the tailwind processor to the build.rs file, a file that compiles as a
separate executable that runs before building your program that does any extra
actions needed for you program to run.

After setting up all that, I found myself some fresh dopamine because the first
page I created looked like this:

![The first-article](/public/images/how-to-make-a-blog-screenie-4.png)

_The subtle off-black coloring. The tasteful red highlights. Oh, my God... It
even has a source code block with syntax highlighting._

I would like to thank _Armin Ronacher_ for the inspiration for this theme.
Please check out [his blog](https://lucumr.pocoo.org/) for great content on web
development and open-source projects.

## Plating the blog

Initially, I wanted to host this on Fly.io, a service that lets you host docker
containers on their servers, but I found that it was a bit hard to get the site
to run on their servers while also letting me connect with a domain name. So I
decided to just host it on a VPS server which I also host Meetballs on.

To host the server, I cloned the repository onto the server and ran the `cargo
build --release` command to build the project. To make the server run and
restart on a crash, I used the `systemd` service manager to run the server as a
service. Here's the actual service file I used to run the server.

```ini
[Unit]
Description=PiresggServer

[Service]
KillSignal=SIGINT
RestartSec=10
User=pedro
Restart=always
WorkingDirectory=/home/pedro/piresgg/server
ExecStart=/home/pedro/piresgg/target/release/blog-server

[Install]
WantedBy=multi-user.target
```

After setting up the service file, I ran the `systemctl enable piresgg-server`
command to enable the service and then ran the `systemctl start piresgg-server`
command to start the server. The server was now running on the VPS server and
I just used cloudflare to point the domain to the server's IP address.

## Some ingredients that I skimmed past

I figured that this website might one day have more than 5 visitors at a time,
so I decided to add it to Cloudflare to cache the site and serve it faster to
the users.

I also added the compression middleware to the server so that the server would
compress the responses before sending them to the client. This is done by
adding the `actix-web` middleware to the server like bellow which uses the gzip
compression algorithm to compress the responses. This is to make the site load
faster and save bandwidth.

```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ...
    HttpServer::new(move || {
        App::new()
            // ...
            .wrap(actix_web::middleware::Compress::default())
            // ...
    })
    // ...
}
```

I know some people who use RSS feeds to read blogs, so I added an RSS feed to
the site so that they could read the posts in their feed reader. It is using
the Atom standard and is available at [/feed.atom](/feed.atom).

Images take a big clump of the bandwidth so on the build.rs file I added a
dependency to compress any images in the `content/images` directory that
copies and compresses the images to the `public/images` directory. This is
done by using the `oxipng`.

Any javascript or css files are also minified and compressed using `esbuild`
every time the server is built in production mode.

## Conclusion

I really like my blog, it is simple, easy to read and easy to write. But it has
some issues I didn't address in this post, like the fact that the server has
no 404 page since I couldn't figure out how to catch the 404 error in the
Actix server. There is no comment section, no search functionality, no tags
and no categories. But those are all planned features that I will try to add
as needed.

I don't exactly know why I made this blog, after attending so many programming
meetups locally and online, I found that there is so many different types of
programmers around the world and I wanted to share my point of view with any
that find themselves struggling with the same issues I have.

I don't know how to end a blog post, but I will get better at it with time.
See you in the next post.
