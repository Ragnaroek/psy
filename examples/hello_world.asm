; a psy port of the gbdev.io tutorial example: https://gbdev.io/gb-asm-tutorial/assets/hello-world.asm
(include :std "gb/dma")

(section .header)
(jp 'rom0)

(section .rom0)
(ld %a 0)
(ld ('hw-sound) %a)
