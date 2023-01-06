import numpy

from typing import Tuple

async def request_frame() -> numpy.ndarray:
    """
    TODO
    """

async def update_frame(image: numpy.ndarray) -> None:
    """
    TODO
    """

async def request_audio() -> Tuple[bytearray, int, int, int]:
    """
    TODO
    """

async def update_audio(
    data: bytearray | bytes, sample_rate: int, sample_width: int, channels: int
) -> None:
    """
    TODO
    """
