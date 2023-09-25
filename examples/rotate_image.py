import asyncio

import cv2

import vioux


async def main():
    image = await vioux.request_frame(0, 0)

    image_flipped = cv2.flip(image["data"], -1)

    # image_flipped = cv2.cvtColor(image_flipped, cv2.COLOR_BGR2RGB)

    # await vioux.update_frame(image["id"], image_flipped, 0, 0)

    id = await vioux.append_frame(0, image_flipped, 0, 0)
    print(id)

    # cv2.imshow("cv2", image["data"])
    # cv2.waitKey(0)


if __name__ == "__main__":
    asyncio.run(main())
