# neutral_color
Finding colors that have the least color contrast with all other colors in the sRGB color space. I'm using this project to learn about coding Rust.

### Current findings:
Currently generates `(109, 110, 66)`: has the __lowest maximum__ color difference of `0.5895155`

After experimentation, `(169, 128, 149)` has the __lowest average__ color difference of `0.2855817`. For comparison, `(109, 110, 66)` has a higher average of `0.34246653`

After experimentation, `(151, 126, 141)` has the __lowest median__ color difference of `0.2771513`. For comparison, `(109, 110, 66)` has a higher median of `0.32988155`