import asyncio

from io import BytesIO

import vioux

import pydub

import pydub.playback


async def main():
    # a = pydub.AudioSegment.from_wav("tests/assets/sound.wav")

    audio = await vioux.request_audio()

    a = pydub.AudioSegment.from_raw(
        BytesIO(audio[0]),
        frame_rate=audio[1],
        sample_width=audio[2],
        channels=audio[3],
    )

    pydub.playback.play(a)


if __name__ == "__main__":
    asyncio.run(main())
