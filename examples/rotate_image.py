import asyncio

import cv2
import vioux


async def main():
    image = await vioux.request_frame()

    # image = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)

    image = cv2.flip(image, -1)

    await vioux.update_frame(image)

    # cv2.imshow("cv2", image)
    # cv2.waitKey(0)


if __name__ == "__main__":
    asyncio.run(main())
