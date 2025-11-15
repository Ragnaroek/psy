(include :std "gb/dma")

(section .rom0)
('start jr 'jump-to)
('start2 jp 'jump-to)
(ds 100)
('jump-to jr 'start)
