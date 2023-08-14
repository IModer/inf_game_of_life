# HashSet of coordinates of live pixels

[
    (0 , 0),
    (0 , 1),
    ...,
]

## Updating

iterate over the pixels:
    koordináta alapján szomszédok megnézése i+[-1,0,1], j+[-1,0,1] akik a vec be vannak
        ha túl sok szomszéd akkor meghal
    koordináta alapján a halott cellák megnézése:
        koordináták alapján a vec ben lévő környezők megnézése:
            ha van elég szomszéd létrejön
