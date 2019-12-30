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
