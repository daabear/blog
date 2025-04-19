---
title: Rust + HTMX blog
slug: rust_htmx_blog
date: 2025-04-18
summary: Creating blog with HTMX, Rust, and Tailwind.
---

### Rust

Using Rust was cool, but it felt like I battling against the complier with the ownership and borrowing system of Rust. I probably should actually (and will) read the entire [Rust book](https://doc.rust-lang.org/book/), instead of just referencing it.

It was easy getting Rust to set up in VSCode. I just needed to install the extensions `rust-analyzer` and the CodeLLDB debugger.

### HTMX

I decided on HTMX for this website because I wanted it to be lightweight and to hit those sweet Lighthouse scores. A frontend framework would be overkill for a simple blog website.

HTMX added interactivity - navigating between content without reloading the page - without writing custom JavaScript. Pretty cool, sorta like a SPA but server-side rendering.

### Tailwind

I didn't want to use a full Node.js build pipeline just to use Tailwind. Tailwind does provide a [standalone CLI without Node](https://tailwindcss.com/blog/standalone-cli), but the guide was dated back to 2021 and wasn't updated to Tailwind v4.

I would suggest this [tutorial](https://github.com/tailwindlabs/tailwindcss/discussions/15855) instead, it helped me set the CLI up and get it running with Tailwind v4.

### Render

I went with [Render](https://render.com/) for deployment due to the simplicity, supporting Rust and not needing any containers.

Although I was disappointed in the limited payment options: no Paypal. But I believe Render uses Stripe as a payment processor, and I get it - you don't really help a competitor.

### Closing

Probably would have been easier and faster making this website using Django, but wanted to try my hand with Rust.

*Anyway, 4 dollars a pound.*
