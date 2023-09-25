from typing import TypedDict

import numpy

class Image(TypedDict):
    data: numpy.ndarray
    x: int
    y: int

class Audio(TypedDict):
    data: bytearray
    sample_rate: int
    sample_width: int
    channels: int
    codec: str

async def request_frame(n: int) -> Image:
    """
    Return a specific frame
    """

async def update_frame(n: int, image: numpy.ndarray, x: int, y: int) -> None:
    """
    Update a specific frame
    """

async def request_audio(n: int) -> Audio:
    """
    Return a specific audio segment
    """

async def update_audio(
    n: int,
    data: bytearray | bytes,
    sample_rate: int,
    sample_width: int,
    channels: int,
    codec: str,
) -> None:
    """
    Update a specific audio segment
    """
