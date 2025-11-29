;TODO address constants
;
;def TILEMAP0 equ $9800 ; $9800-$9BFF
;def TILEMAP1 equ $9C00 ; $9C00-$9FFF

(def-section .tile-map-0
    :offset 0x9800
    :label-only true)
(section .tile-map-0)
('tile-map-0 db)

(def-section .tile-map-1
    :offset 0x9C00
    :label-only true)

(def-section .header
    :offset 0x100
    :length 0x50)

; a ROM has at least to 16 KiB Banks, always named here
; rom0 and rom1
(def-section .rom0
    :offset 0x150
    :length 0x4000) ; 16 KiB
(section .rom0)
('rom0 db)

(def-section .rom1
    :offset 0x4150
    :length 0x4000) ; 16 KiB
(section .rom1)
('rom1 db)

(def-section .vram
    :offset 0x8000
    :length 0x1FFF
    :label-only true)
(section .vram)
('vram db)
