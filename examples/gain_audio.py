import asyncio

from io import BytesIO

import vioux

import pydub

import pydub.playback


async def main():
    audio = await vioux.request_audio()

    a = pydub.AudioSegment.from_raw(
        BytesIO(audio[0]),
        frame_rate=audio[1],
        sample_width=audio[2],
        channels=audio[3],
    )

    await vioux.update_audio(
        bytearray(a.raw_data), a.frame_rate, a.sample_width, a.channels
    )

    # pydub.playback.play(a)


if __name__ == "__main__":
    asyncio.run(main())
