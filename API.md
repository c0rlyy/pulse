
# Api docs showcasing example api usage:

project by default uses comand cargo run in docker container which makes it start in developer mode
meaning it can be much slower, specificly with hashing and verifiying password up to even 10 times,
when project will have more feautres and complete frontend in the main branch there will be fully optimized build,
keep in mind that all endpoint might drasticly change in future

## API Endpoints

### 1. Create User

**Endpoint:** `/api/users`  
**Method:** `POST`

**Request Body:**

```json
{
  "username": "new_user",
  "email": "new_user@example.com",
  "password": "password123",
  "is_private": false
}
```
**response**
```json
{
  "id": 1,
  "username": "new_user",
  "is_active": true,
  "is_private": false
}
```

### 2. Login

**Endpoint:** `/api/login`  
**Method:** `POST`

**Request Body:**

```json
{
  "client_id": "your_email@example.com",
  "client_secret": "your_password"
}
```
**response:**
```json
{
  "access_token": "your_jwt_token",
  "token_type": "Bearer"
}
```
### 3. Get Current User

**Endpoint:** `/api/users/me`  
**Method:** `GET`

**Headers:**

```http
Authorization: Bearer your_jwt_token
```

**response**
```json
{
  "id": 1,
  "username": "example_user",
  "is_active": true,
  "is_private": false,
  "role": "USER"
}
```
### 4. Get All Users
***this currently remains to have a way to easely see user entities will be removed in future***

**Endpoint:** `/api/users`  
**Method:** `GET`

**Headers:**

```http
Authorization: Bearer your_jwt_token
```

**response**
```json
{
  "id": 1,
  "username": "example_user",
  "email": "example@email.com",
  "password": "im_hashed",
  "is_active": true,
  "is_private": false,
  "role": "USER"
}
```

### 5. Delete User

**Endpoint:** `/api/users`  
**Method:** `DELETE`

**headers:**

```http
authorization: bearer your_jwt_token
```

**Body:**

```json
{
  "password": "im_hashed",
}

**response**
```http
Response 200 OK
```

### 6. Search users, exludes private and inactive accounts
**Endpoint:** `/api/users/search?username=user&limit=3&skip=0`  
**Method:** `GET`


**Body:**

**response**
```json
[
    {
        "id": 1,
        "username": "userna",
        "is_active": true,
        "is_private": false
    },
    {
        "id": 2,
        "username": "usernam",
        "is_active": true,
        "is_private": false
    },

    {
        "id": 3,
        "username": "username",
        "is_active": true,
        "is_private": false
    },

]

```

### 7. Update user password

***Endpoint:*** `/api/users/password`
**Method:** `PATCH`

**headers:**

```http
authorization: bearer your_jwt_token
```
***Request Body:***

```json
{
    "password": "current_password",
    "new_password": "new_password"
}

```

**response**
```http
Response 200 OK
```

### 8. Update user email

***Endpoint:*** `/api/users/email`
**Method:** `PATCH`

**headers:**

```http
authorization: bearer your_jwt_token
```
***Request Body:***

```json
{
    "password": "current_password",
    "new_email": "new_password"
}

```

**response**
```http
Response 200 OK
```
