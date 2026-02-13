from .api import NavPipe
import importlib.metadata

try:
    __version__ = importlib.metadata.version("navpipe")
except importlib.metadata.PackageNotFoundError:
    __version__ = "0.1.0"


__all__ = ["NavPipe"]
