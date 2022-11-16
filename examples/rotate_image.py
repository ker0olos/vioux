import asyncio

import cv2

import vioux


async def main():
    image = await vioux.request()

    image = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)

    cv2.imshow("cv2", image)
    cv2.waitKey(0)


if __name__ == "__main__":
    asyncio.run(main())
