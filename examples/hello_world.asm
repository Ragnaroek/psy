; a psy port of the gbdev.io tutorial example: https://gbdev.io/gb-asm-tutorial/assets/hello-world.asm
(include :std "gb/dma")

(section .header)
(jp 'rom0)

(section .rom0)
; Shut down audio circuitry
(ld %a 0)
(ld ('hw-sound) %a)

;Do not turn the LCD off outside of VBlank
(label 'wait-vb-blank)
(ld %a ('hw-ly))
