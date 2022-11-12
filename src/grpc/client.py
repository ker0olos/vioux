from grpc_requests import Client


def run():
    client = Client.get_by_endpoint("127.0.0.1:50051")

    request_data = {"nonce": "1"}

    result = client.request("placeholder.Placeholder", "SendMessage", request_data)

    print(result)


if __name__ == "__main__":
    run()
