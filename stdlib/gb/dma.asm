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
    :offset 0x9C00)

(def-section .header
    :offset 0x100
    :length 0x50)

(def-section .rom0
    :offset 0x150
    :length 100)
