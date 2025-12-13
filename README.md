# Scheduler API Documentation

API untuk sistem scheduler dengan fitur autentikasi JWT dan manajemen user.
> Write on rust for efficiency and better peformance

## Base URL

```
{{base_url}}
```

## Authentication

API ini menggunakan JWT (JSON Web Token) untuk autentikasi. Token yang digunakan:
- **Access Token**: Digunakan untuk mengakses endpoint yang memerlukan autentikasi
- **Refresh Token**: Digunakan untuk mendapatkan access token baru

### Headers untuk Authenticated Endpoints

```
Authorization: Bearer {access_token}
```

## Endpoints

### 1. Authentication

#### Login

Melakukan login dan mendapatkan access token & refresh token.

**Endpoint:** `POST /login`

**Request Body:**
```json
{
    "identifier": "admin",
    "password": "admin123"
}
```

**Response Success (200):**
```json
{
    "success": true,
    "status": 200,
    "message": "Login success!",
    "path": "/login",
    "timestamp": "2025-12-09T06:58:02Z",
    "data": {
        "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
        "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
    }
}
```

---

#### Refresh Token

Mendapatkan access token dan refresh token baru.

**Endpoint:** `POST /refresh/token`

**Request Body:**
```json
{
    "refresh_token": "{{refresh_token}}"
}
```

**Response Success (200):**
```json
{
    "success": true,
    "status": 200,
    "message": "Refresh success!",
    "path": "/refresh/token",
    "timestamp": "2025-12-09T06:59:14Z",
    "data": {
        "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
        "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
    }
}
```

---

#### Logout

Melakukan logout dan menginvalidasi token.

**Endpoint:** `POST /logout`

**Authentication:** Required (Bearer Token)

**Request Body:**
```json
{
    "refresh_token": "{{refresh_token}}"
}
```

**Response Success (200):**
```json
{
    "success": true,
    "status": 200,
    "message": "Logout successful!",
    "path": "/logout",
    "timestamp": "2025-12-09T11:12:29Z"
}
```

---

### 2. User Management

#### Get Profile

Mendapatkan informasi profile user yang sedang login.

**Endpoint:** `GET /user/me`

**Authentication:** Required (Bearer Token)

**Response Success (200):**
```json
{
    "success": true,
    "status": 200,
    "message": "Welcome admin!",
    "path": "/user/me",
    "timestamp": "2025-12-11T13:41:06Z",
    "data": {
        "id": 1,
        "username": "admin",
        "email": "admin@teknohole.com",
        "is_superuser": true,
        "created_at": "2025-12-09T03:01:40.912343Z",
        "updated_at": "2025-12-11T13:41:00.158809Z"
    }
}
```

---

#### Update Profile

Memperbarui informasi profile user yang sedang login.

**Endpoint:** `PATCH /user/me`

**Authentication:** Required (Bearer Token)

**Request Body:**
```json
{
    "username": "admin",
    "email": "admin@teknohole.com",
    "is_superuser": true
}
```

**Response Success (200):**
```json
{
    "success": true,
    "status": 200,
    "message": "Profile updated successfully!",
    "path": "/user/me",
    "timestamp": "2025-12-11T13:41:00Z",
    "data": {
        "id": 1,
        "username": "admin",
        "email": "admin@teknohole.com",
        "is_superuser": true,
        "created_at": "2025-12-09T03:01:40.912343Z",
        "updated_at": "2025-12-11T13:41:00.158809Z"
    }
}
```

---

#### Get All Users

Mendapatkan daftar semua user (hanya untuk superuser).

**Endpoint:** `GET /user`

**Authentication:** Required (Bearer Token)

**Response Success (200):**
```json
{
    "success": true,
    "status": 200,
    "message": "List of all users",
    "path": "/user",
    "timestamp": "2025-12-13T04:14:14Z",
    "data": [
        {
            "id": 1,
            "username": "admin",
            "email": "admin@teknohole.com",
            "is_superuser": true,
            "created_at": "2025-12-09T03:01:40.912343Z",
            "updated_at": "2025-12-13T04:07:34.331875Z"
        },
        {
            "id": 2,
            "username": "arbath",
            "email": "arbath@teknohole.com",
            "is_superuser": false,
            "created_at": "2025-12-13T04:09:29.647656Z",
            "updated_at": "2025-12-13T04:09:29.647656Z"
        }
    ]
}
```

---

#### Create User

Membuat user baru (hanya untuk superuser).

**Endpoint:** `POST /user`

**Authentication:** Required (Bearer Token)

**Request Body:**
```json
{
    "username": "arbath",
    "password": "arbath123",
    "email": "arbath@teknohole.com",
    "is_superuser": false
}
```

**Response Success (201):**
```json
{
    "success": true,
    "status": 201,
    "message": "User arbath has been created successfully!",
    "path": "/user",
    "timestamp": "2025-12-13T04:14:04Z",
    "data": {
        "id": 2,
        "username": "arbath",
        "email": "arbath@teknohole.com",
        "is_superuser": false,
        "created_at": "2025-12-13T04:14:04.051047Z",
        "updated_at": "2025-12-13T04:14:04.051047Z"
    }
}
```

---

#### Delete User

Menghapus user berdasarkan ID (hanya untuk superuser).

**Endpoint:** `DELETE /user/{id}`

**Authentication:** Required (Bearer Token)

**Path Parameters:**
- `id` (integer): ID user yang akan dihapus

**Response Success (200):**
```json
{
    "success": true,
    "status": 200,
    "message": "User with id 5 has been deleted successfully!",
    "path": "/user/5",
    "timestamp": "2025-12-13T04:14:39Z"
}
```

---

### 3. Other

#### Home

Endpoint untuk mengecek status API.

**Endpoint:** `GET /`

**Authentication:** Not Required

---

## Response Format

Semua response mengikuti format standar:

```json
{
    "success": boolean,
    "status": number,
    "message": string,
    "path": string,
    "timestamp": string,
    "data": object | array (optional)
}
```

## Error Responses

API akan mengembalikan response error dengan format yang sama:

- **400 Bad Request**: Request body tidak valid
- **401 Unauthorized**: Token tidak valid atau tidak ada
- **403 Forbidden**: User tidak memiliki akses
- **404 Not Found**: Resource tidak ditemukan
- **500 Internal Server Error**: Kesalahan server

## Notes

- Semua timestamp menggunakan format ISO 8601 (UTC)
- Access token dan refresh token harus disimpan dengan aman
- Token akan expired sesuai dengan konfigurasi server
- Endpoint yang memerlukan superuser hanya dapat diakses oleh user dengan `is_superuser: true`

>by: arbath@teknohole.com