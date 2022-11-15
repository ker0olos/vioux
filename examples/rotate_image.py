import asyncio

import cv2

import vioux


async def main():
    image = await vioux.request()

    image = cv2.imdecode(image, cv2.IMREAD_COLOR)

    cv2.imshow("cv2", image)
    cv2.waitKey(0)


if __name__ == "__main__":
    asyncio.run(main())
