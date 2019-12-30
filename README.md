# throwaway-check-doi
Sample to test rust regex parsing (throwaway code).

```shell
$ throwaway-check-doi
Hello => fail
World => fail
10.1234/aksjdfh abz => fail
10.1234/aksjdfh => ok
10.25513/1812-3996.2017.1.34–42 => fail
10.25513/1812-3996.2017.1.34 => ok
10.25513/1812-3996.2017.1.34- => ok
10.25513/1812-3996.2017.1.34-42 => ok
```

* [https://www.unicode.org/Public/UCD/latest/ucd/PropList.txt](https://www.unicode.org/Public/UCD/latest/ucd/PropList.txt)

> // 2000..200A    ; White_Space # Zs  [11] EN QUAD..HAIR SPACE
