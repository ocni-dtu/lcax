from lcax import Source


def test_source_access(source):
    assert source
    assert source.name == "Ökobau"
    assert not source.url

def test_new_source():
    source = Source(name="LCAx")
    assert source
    
    assert source.name == "LCAx"
    
    
def test_source_str(source):
    assert str(source) == "Source: Ökobau"

def test_source_subclass():
    class MySource(Source):
        pass

    source = MySource(name="LCAx")
    assert source