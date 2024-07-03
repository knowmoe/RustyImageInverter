### Rusty Image Inverter

## Usage

```
.\RustImageInverter.exe <input file> <output file>
```

e.g.:

```
PS E:\RustyImageInverter> ls img


    Directory: E:\RustyImageInverter\img


Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a----        03/07/2024     02:28          19716 aegis.png


PS E:\RustyImageInverter> .\RustyImageInverter.exe .\img\aegis.png .\img\aegis_inverted.png
```

With output:

```
PS E:\RustyImageInverter> ls img


    Directory: E:\RustyImageInverter\img


Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a----        03/07/2024     02:28          19716 aegis.png
-a----        03/07/2024     03:02          47052 aegis_inverted.png
```

## Example Images

### Original Image

![Original Image](./img/aegis.png)

### Inverted Image

![Inverted Image](./img/aegis_inverted.png)

## Note

This has only been tested with `.png` files so far :p

