# IDK WHAT TO NAME THIS
This project was heavily inspired by [pfetch](https://github.com/dylanaraps/pfetch). After using pfetch for a while, some parts started bothering me. First, it's written in shell. It takes a noticeable amount of time to run, which is unreasonable for something that is simply reading files and printing out some formatted results. I've solved this problem by making my rewrite in blazingly-fast Rust. Second, it has some completely useless information. I've replace "host" with "ip," and "pkgs" with either "power" (for laptops) or "cpu" (for desktops and servers).

## How to install
```shell
git clone https://github.com/MiningTcup/thiccfetch.git
cd thiccfetch
cargo build --release
chmod +x target/releases/thiccfetch
sudo cp target/release/thiccfetch /usr/bin/thiccfetch
```

## Contributing
Feel free to make an issue suggesting a name, or a pull request fixing my dumb code, adding support for an OS, or anything else. If you add a new OS, use the icon from pfetch, unless you think you can do better than what they have.
