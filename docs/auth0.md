# Auth0 Docs

## Local Database Setup

To run a local database, we use a `docker-compose.yaml` file. This sets up a persistent PostgreSQL database and also launches a web UI via pgAdmin.

To start the setup, run:

```
docker compose up -d
```

For more information, refer to the `docker-compose.yaml` file.

---

<br>

## Running Auth0 in Development

When running the project locally, some additional configuration is required for Auth0 to function properly:

### 1. Webhook for User Creation

To enable user creation in **your own database** (not in Auth0), a webhook from Auth0 must reach your backend. Since your backend is running locally, you need to expose it using **ngrok**.

You can do this with the `just` command:

```
just ngrok
```

> 💡 It’s recommended to run this in a **separate terminal** from the backend process.

---

### 2. Update the Auth0 Trigger

When ngrok starts, it generates a new random public URL (on the free tier). As a result, every time you restart ngrok, you must update the webhook URL used in the Auth0 Action.

To do this:

- Go to the **Auth0 Dashboard**
- Navigate to **Actions > Library**
- Open the action named **`Postgres user creation`**
- Update the `ngrok` URL variable in the JavaScript code with the new public URL

---

### 3. Frontend Callback URLs

Make sure to configure the **Allowed Callback URLs** in your Auth0 dashboard for the frontend to handle authentication properly. Refer to the Auth0 documentation for how to set these up.
