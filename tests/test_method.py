class Class:
    def m(self, x: int) -> str:
        return str(x)

    @staticmethod
    def s(x: int) -> str:
        return str(x)

    @classmethod
    def c_int(cls, x: int) -> int:
        return x

    @classmethod
    def c_str(cls, x: int) -> str:
        return str(x)

    @classmethod
    def c_none(cls, x: int) -> None:
        return

    @classmethod
    def c_noann(cls, x: int):
        return

    @classmethod
    def c_two(cls, x: int, y: int) -> int:
        return x + y

    @classmethod
    def c_two_var(cls, *args: int, **kwargs: int) -> int:
        return sum(args) + sum(kwargs.values())

    @classmethod
    def c_pkwarg(cls, x: int, y: int = 1) -> int:
        return x + y


def test_is_template_method_normal():
    from textconf.template import is_template_method

    assert not is_template_method(Class.m)


def test_is_template_method_static():
    from textconf.template import is_template_method

    assert not is_template_method(Class.s)


def test_is_template_method_int():
    from textconf.template import is_template_method

    assert is_template_method(Class.c_int)


def test_is_template_method_str():
    from textconf.template import is_template_method

    assert is_template_method(Class.c_str)


def test_is_template_method_none():
    from textconf.template import is_template_method

    assert not is_template_method(Class.c_none)


def test_is_template_method_noann():
    from textconf.template import is_template_method

    assert not is_template_method(Class.c_noann)


def test_is_template_method_two():
    from textconf.template import is_template_method

    assert not is_template_method(Class.c_two)


def test_is_template_method_two_var():
    from textconf.template import is_template_method

    assert not is_template_method(Class.c_two_var)


def test_is_template_method_pkwarg():
    from textconf.template import is_template_method

    assert not is_template_method(Class.c_pkwarg)


def test_iter_template_methods():
    from textconf.template import iter_template_methods

    it = iter_template_methods(Class)
    assert next(it) == ("c_int", Class.c_int)
    assert next(it) == ("c_str", Class.c_str)
    assert len(list(iter_template_methods(Class))) == 2
