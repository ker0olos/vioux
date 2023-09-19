import asyncio
from io import BytesIO

from pydub import AudioSegment
from pydub.playback import play

import vioux


async def main():
    audio = await vioux.request_audio(0)

    print(audio[1], audio[2], audio[3], audio[4])

    # t = AudioSegment.from_raw(
    #     BytesIO(audio[0]),
    #     frame_rate=audio[1],
    #     sample_width=audio[2],
    #     channels=audio[3],
    # )

    await vioux.update_audio(0, audio[0], audio[1], audio[2], audio[3], audio[4])

    # play(t)


if __name__ == "__main__":
    asyncio.run(main())
