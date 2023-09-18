import asyncio

import vioux


async def main():
    audio = await vioux.request_audio(0)

    # print(audio[4] / audio[2])

    # a = pydub.AudioSegment.from_raw(
    #     BytesIO(audio[0]),
    #     frame_rate=audio[1],
    #     sample_width=audio[2],
    #     channels=audio[3],
    # )

    # await vioux.update_audio(
    #     bytearray(a.raw_data), a.frame_rate, a.sample_width, a.channels
    # )

    # pydub.playback.play(a)


if __name__ == "__main__":
    asyncio.run(main())
