import asyncio
from io import BytesIO

from pydub import AudioSegment
from pydub.playback import play

import vioux


async def main():
    audio = await vioux.request_audio(0)

    print(
        audio["sample_rate"], audio["sample_width"], audio["channels"], audio["codec"]
    )

    # t = AudioSegment.from_raw(
    #     BytesIO(audio["data"]),
    #     frame_rate=audio["sample_rate"],
    #     sample_width=audio["sample_width"],
    #     channels=audio["channels"],
    # )

    await vioux.update_audio(
        0,
        audio["data"],
        audio["sample_rate"],
        audio["sample_width"],
        audio["channels"],
        audio["codec"],
    )

    # play(t)


if __name__ == "__main__":
    asyncio.run(main())
