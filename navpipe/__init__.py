import importlib.metadata
from .navpipe import NavPipe

try:
    __version__ = importlib.metadata.version("navpipe")
except importlib.metadata.PackageNotFoundError:
    __version__ = "0.2.0"


__all__ = ["NavPipe"]