# Pass
A linux password manager written in Rust using GTK and Libadwaita.

## Building
### With Nix
```bash
git clone https://github.com/justryanw/pass
cd pass
nix develop
cargo run --release
```

### Without Nix
Ensure that you have installed [Rust](https://www.rust-lang.org/tools/install), the dependencies for [gtk4-rs](https://gtk-rs.org/gtk4-rs/git/book/installation.html) and [libadwaita](https://gtk-rs.org/gtk4-rs/git/book/libadwaita.html#linux), and [pkg-config](https://www.freedesktop.org/wiki/Software/pkg-config/).
```bash
git clone https://github.com/justryanw/pass
cd pass
cargo run --release
```

## Screenshots

### Main Page
The main page is shows a sidebar and the main content.

![](./images/4.png)

### Limited Width
With a smaller window the page folds to only show the sidebar, clicking on an item brings you to its content with a back button to return.

![](./images/5.png)
![](./images/6.png)

### Placeholder Pages
Placeholder pages guide the user in inital setup.

![](./images/1.png)
![](./images/2.png)
![](./images/3.png)