import asyncio

from io import BytesIO

import vioux

import pydub

import pydub.playback


async def main():
    # a = pydub.AudioSegment.from_wav("tests/assets/sound.wav")

    # samples = a.get_array_of_samples()

    # print(len(samples) / 2)
    # print(samples[::-1])

    audio = await vioux.request_audio()

    # print(audio[0][::-1])
    # print(len(audio[0]) / 2)
    # print((len(audio[0]) / 2) == 96768)

    a = pydub.AudioSegment.from_raw(
        BytesIO(audio[0]),
        frame_rate=audio[1],
        sample_width=audio[2],
        channels=audio[3],
    )

    pydub.playback.play(a)

    # print(a)

    ...


if __name__ == "__main__":
    asyncio.run(main())
