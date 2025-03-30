from .lcax import *
import lcax as lcax_binary


__doc__ = lcax_binary.__doc__
if hasattr(lcax_binary, "__all__"):
    __all__ = lcax_binary.__all__
