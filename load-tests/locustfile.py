from locust import HttpUser, task
import random

class UserLoadTest(HttpUser):
    @task(3)
    def get_user(self):
        user_id = random.randrange(1, 10000)
        self.client.get(f"/users/{user_id}", name="/item")
