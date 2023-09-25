from typing import TypedDict

import numpy

class Image(TypedDict):
    id: str
    data: numpy.ndarray
    x: int
    y: int

class Audio(TypedDict):
    id: str
    data: bytearray
    sample_rate: int
    sample_width: int
    channels: int
    codec: str

async def request_frame(layer: int, frame: int) -> Image:
    """
    Return a specific frame
    """

async def update_frame(id: str, image: numpy.ndarray, x: int, y: int) -> None:
    """
    Update a specific frame
    """

async def append_frame(layer: int, image: numpy.ndarray, x: int, y: int) -> None:
    """
    Append a new frame
    """

async def request_audio(layer: int, n: int) -> Audio:
    """
    Return a specific audio segment
    """

async def update_audio(
    id: str,
    data: bytearray | bytes,
    sample_rate: int,
    sample_width: int,
    channels: int,
    codec: str,
) -> None:
    """
    Update a specific audio segment
    """

async def append_audio(
    layer: int,
    data: bytearray | bytes,
    sample_rate: int,
    sample_width: int,
    channels: int,
    codec: str,
) -> str:
    """
    Append a new audio segment
    """
