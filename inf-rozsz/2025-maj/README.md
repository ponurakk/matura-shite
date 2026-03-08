## 2.1
Podaj wszystkie takie napisy z pliku `symbole.txt`, które są palindromami (czytane od przodu i od tyłu są takie same). Wypisz je po jednym w wierszu, w kolejności takiej jak
w pliku `symbole.txt`.

Odpowiedź dla pliku `symbole_przyklad.txt` to
```
oooo+**+oooo
```
(w tym pliku jest jeden palindrom)

## 2.2
W pliku `symbole.txt` szukamy „kwadratów” złożonych z dziewięciu sąsiadujących
identycznych symboli:
<div align="center">
<pre>
+ + +       o o o       * * *
+ + +  lub  o o o  lub  * * *
+ + +       o o o       * * *
</pre>
</div>

Podaj, ile takich kwadratów występuje w pliku `symbole.txt`. Jeżeli w pliku występuje
jeden taki kwadrat, podaj numer wiersza i numer pozycji w wierszu (licząc od 1) jego
**środkowego pola**. Jeżeli jest więcej takich kwadratów, podaj numer wiersza i numer pozycji
w wierszu dla **środkowego pola**  każdego z nich.
**Przykład:** 
Poniżej podano 6 wierszy przykładowych danych (po 12 znaków w każdym wierszu):
```
1. + * * + o * o + + * o +
2. + + + o o o o * o * * *
3. + o * o o o o * * + + +
4. * + * o o o o o o + + +
5. o * * o + + + o + + + +
6. o o o o + + * * + * + o
```
Mamy tutaj trzy kwadraty złożone z 9 identycznych symboli: pierwszy ma środek w wierszu
3 na pozycji 5, drugi – w wierszu 3 na pozycji 6, a trzeci – w wierszu 4 na pozycji 11.
Odpowiedź dla pliku `symbole_przyklad.txt` to
1 6 3
(jeden kwadrat, który ma środkowe pole w wierszu 6, na pozycji 3).

> [!NOTE]
> **Informacja do zadań 2.3. i 2.4**
>
> Każdy z napisów podanych w pliku `symbole.txt` będziemy traktować jako liczbę
> zapisaną w systemie trójkowym, w którym:
>
> - znak `o` odpowiada cyfrze `0`
> - znak `+` odpowiada cyfrze `1`
> - znak `*` odpowiada cyfrze `2`

## 2.3
Podaj największą liczbę spośród liczb zapisanych w pliku `symbole.txt`. W odpowiedzi
podaj tę liczbę w zapisie dziesiętnym oraz napis jej odpowiadający.
Odpowiedź dla pliku `symbole_przyklad.txt` to
```
519789 ***+o*ooo++o
```

## 2.4
Oblicz sumę wszystkich liczb z pliku `symbole.txt`. Podaj jej wartość w zapisie
dziesiętnym oraz w zapisie trójkowym z użyciem symboli: `o, +, *`.
Odpowiedź dla pliku `symbole_przyklad.txt` to
```
4841542 +oooo****+oo+o+
```


# Odpowiedzi

## 2.1
```
++o+o++o+o++
+*+**++**+*+
*+o++**++o+*
*oo*o**o*oo*
+*++*oo*++*+
+o++oooo++o+
```

## 2.2
```
3 399 5 546 2 630 11
(3 kwadraty o środkach (399, 5) (546, 2) (630, 11))
```

## 2.3
```
531246 *******o+*+o
```

## 2.4
```
527865439 ++oo*+oo*++ooo*o*++
```
