//! Part 3: TODO
//! 
//!   - **Date:** May 26, 2017
//!   - **Subject:** TODO
//!   - **Audio:**
//!       + [M4A][m4a]
//!       + [MP3][mp3]
//!       + [Ogg][ogg]
//!
//! [m4a]: http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/jonathan_turner_part_3.m4a
//! [mp3]: http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/jonathan_turner_part_3.mp3
//! [ogg]: http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/jonathan_turner_part_3.ogg
//!
//! <audio style="width: 100%" style="width: 100%" title="Jonathan Turner::Part 3" controls preload=metadata>
//!   <source src="http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/jonathan_turner_part_3.m4a">
//!   <source src="http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/jonathan_turner_part_3.mp3">
//!   <source src="http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/jonathan_turner_part_3.ogg">
//! </audio>
//!
//!
//! Show Notes
//! ----------
//! 
//!   - The [survey]
//!   - Language adoption:
//!       + [Guido van Rossum] and [Python]
//!       + [Matz (Yukihiro Matsumoto)][Matz] and [Ruby]
//!       + [Dart]
//!       
//! [design issue]: https://github.com/rust-lang/rust/issues/33240
//! ["Helping with the Rust Errors"]: http://www.jonathanturner.org/2016/08/helping-out-with-rust-errors.html
//! ["Shape of Errors to Come"]: https://blog.rust-lang.org/2016/08/10/Shape-of-errors-to-come.html
//! [Elm]: http://elm-lang.org
//! [error list issue]: https://github.com/rust-lang/rust/issues/35233
//! [survey]: https://blog.rust-lang.org/2016/06/30/State-of-Rust-Survey-2016.html
//! [Guido van Rossum]: https://en.wikipedia.org/wiki/Guido_van_Rossum
//! [Python]: https://www.python.org
//! [Matz]: https://en.wikipedia.org/wiki/Yukihiro_Matsumoto
//! [Ruby]: https://www.ruby-lang.org/en/
//! [Dart]: https://www.dartlang.org
//! 
//! Building the Rust Language Service:
//! 
//!   - [Racer]
//!   - [rustw]
//!   - [Language Server Protocol][lsp]
//!   - [Demo at RustConf 2016]
//!   - [Anders Hejlsberg] – designer or lead developer of [Turbo Pascal],
//!     [Delphi], [C#], and [TypeScript]
//!   - [Serde]
//!   - [Roadmap GitHub Project]
//!   - Language Server Protocol plugins
//!       + [RLS reference VS Code plugin]
//!       + Kalita Alexey's [vscode-rust]
//!       + [langserver.org]
//!   - [The 2017 Rust Roadmap]
//!       + [Improved match ergonomics around references]
//!       + [const generics]
//!           * [RFC #1931]
//! 
//! [Racer]: https://github.com/phildawes/racer
//! [rustw]: https://github.com/nrc/rustw
//! [lsp]: https://github.com/Microsoft/language-server-protocol
//! [Demo at RustConf 2016]: https://youtu.be/pTQxHIzGqFI?t=42m5s
//! [Anders Hejlsberg]: https://en.wikipedia.org/wiki/Anders_Hejlsberg
//! [Turbo Pascal]: https://en.wikipedia.org/wiki/Turbo_Pascal
//! [Delphi]: https://en.wikipedia.org/wiki/Delphi_(programming_language)
//! [C#]: https://docs.microsoft.com/en-us/dotnet/articles/csharp/
//! [TypeScript]: http://www.typescriptlang.org
//! [Serde]: https://serde.rs
//! [Roadmap GitHub Project]: https://github.com/rust-lang-nursery/rls/projects/2
//! [RLS reference VS Code plugin]: https://github.com/jonathandturner/rls_vscode
//! [vscode-rust]: https://github.com/editor-rs/vscode-rust
//! [langserver.org]: http://langserver.org
//! [The 2017 Rust Roadmap]: https://blog.rust-lang.org/2017/02/06/roadmap.html
//! [Improved match ergonomics around references]: https://github.com/rust-lang/rust-roadmap/issues/24
//! [const generics]: https://internals.rust-lang.org/t/lang-team-minutes-const-generics/5090
//! [RFC #1931]: https://github.com/rust-lang/rfcs/pull/1931
//! 
//! 
//! Working on Servo:
//! 
//!   - [Servo]
//!       + [Windows nightlies]
//!   - [LLVM]
//!       + Apple's use on their graphics pipeline:
//!           - [OpenGL]
//!           - [Metal]
//!       + [clang]
//!       + [Swift]
//!   - [Project Quantum]
//!   - [WebKit]
//!       + [KHTML]
//!       + [Safari]
//!   
//! [Servo]: https://servo.org
//! [Windows nightlies]: https://blog.servo.org/2017/04/13/windows/
//! [llvm]: http://llvm.org
//! [OpenGL]: http://lists.llvm.org/pipermail/llvm-dev/2006-August/006497.html
//! [Metal]: https://developer.apple.com/metal/metal-shading-language-specification.pdf
//! [clang]: http://clang.llvm.org
//! [swift]: https://swift.org
//! [Project Quantum]: https://medium.com/mozilla-tech/a-quantum-leap-for-the-web-a3b7174b3c12
//! [WebKit]: https://webkit.org
//! [KHTML]: https://en.wikipedia.org/wiki/KHTML
//! [Safari]: https://www.apple.com/safari/
//! 
//! 
//! Sponsors
//! --------
//! 
//!   - [Anthony Deschamps]
//!   - Anthony Scotti
//!   - Aleksey Pirogov
//!   - Andreas Fischer
//!   - Andrew Thompson
//!   - Austin LeSure
//!   - Ben Whitley
//!   - [Charlie Egan]
//!   - Chris Jones
//!   - [Chris Palmer]
//!   - [Christopher Giffard]
//!   - Dan Abrams
//!   - [Daniel Collin]
//!   - [Derek Morr]
//!   - Eugene Bulkin
//!   - [Henri Sivonen]
//!   - [Jakub "Limeth" Hlusička]
//!   - Jonathan Turner
//!   - Jordan Henderson
//!   - [Jupp Müller]
//!   - Justin Ossevoort
//!   - [Karl Hobley]
//!   - Keith Gray
//!   - Kilian Rault
//!   - Lachlan Collins
//!   - Luca Schmid
//!   - Matt Rudder
//!   - Matthew Piziak
//!   - [Max Jacobson]
//!   - Micael Bergeron
//!   - Ovidiu Curcan
//!   - [Pascal Hertleif]
//!   - [Patrick O'Doherty]
//!   - Peter Tillemans
//!   - Philipp Keller
//!   - Ralph Giles ("rillian")
//!   - Raph Levien
//!   - reddraggone9
//!   - Steven Murawksi
//!   - [Stuart Hinson]
//!   - Ty Overby
//!   - Tyler Harper
//!   - Vesa Kaihlavirta
//!   - Warren Harper
//!   - [William Roe]
//!   - Zaki
//!
//! [Anthony Deschamps]: https://github.com/adeschamps
//! [Charlie Egan]: https://charlieegan3.com
//! [Chris Palmer]: http://home.red-oxide.org/
//! [Christopher Giffard]: http://blog.cgiffard.com
//! [Daniel Collin]: https://twitter.com/daniel_collin
//! [Derek Morr]: https://twitter.com/derekmorr
//! [Henri Sivonen]: https://hsivonen.fi/
//! [Jakub "Limeth" Hlusička]: https://github.com/Limeth
//! [Jupp Müller]: https://de.linkedin.com/in/juppm
//! [Karl Hobley]: https://github.com/kaedroho/
//! [Max Jacobson]: https://twitter.com/maxjacobson
//! [Pascal Hertleif]: https://pascalhertleif.de/
//! [Patrick O'Doherty]: https://twitter.com/patrickod
//! [Philipp Keller]: https://twitter.com/hansapla
//! [Stuart Hinson]: http://stuarth.github.io/
//! [William Roe]: http://willroe.me
//!
//! (Thanks to the couple people donating who opted out of the reward tier, as
//! well. You know who you are!)
//!
//! ### Become a sponsor
//!
//!   - [Patreon](https://www.patreon.com/newrustacean)
//!   - [Venmo](https://venmo.com/chriskrycho)
//!   - [Dwolla](https://www.dwolla.com/hub/chriskrycho)
//!   - [Cash.me](https://cash.me/$chriskrycho)
//!   - [Flattr](https://flattr.com/profile/chriskrycho)
//!   - [PayPal.me](https://paypal.me/chriskrycho)
//!
//!
//! Contact
//! -------
//!
//!   - New Rustacean:
//!       + Twitter: [@newrustacean](https://www.twitter.com/newrustacean)
//!       + Email: [hello@newrustacean.com](mailto:hello@newrustacean.com)
//!   - Chris Krycho
//!       + GitHub: [chriskrycho](https://github.com/chriskrycho)
//!       + Twitter: [@chriskrycho](https://www.twitter.com/chriskrycho)
