# WebserverChallenge



# Original Instructions
**Rust Web Server**
Using Warp or Actix, create a web server that interacts with an external web service.  Show off your skills and make this production-quality (this part is intentionally open-ended and not defined below).  To reiterate, try to go beyond what is being asked below.  For example, add a well-formed README, test all-the-things, and create a Config struct shared in a lazy static.External Web Service
A third-party data feed from https://jsonplaceholder.typicode.com/ will be used for this assignment.Endpoints


**1. List Users**
Request: GET /api/v1/users
Response: 200 OK

[
  {
    "id": 1,
    "name": "Leanne Graham",
    "email": "Sincere@april.biz",
  },
  {
    "id": 2,
    "name": "Ervin Howell",
    "email": "Shanna@melissa.tv",
  },
  ...
]

**2. Create a User**
Request #1 - Invalid Payload
This request contains parameters that intentionally fail to satisfy validation requirements.
The name will need to be at least 3 characters long and the email address should be valid.
Validate the length of the name and the validity of the email address.
Intentionally trigger the error case below.Request: POST /api/v1/users

{ "name": "MF", "email": "bad" }

Response: 422 Unprocessable Entity

{ "errors": ["Invalid email address"] }

Request #2 - Valid Payload
A valid request that meets all parameter requirements above.
Request: POST /api/v1/users

{ "name": "Martin Fowler", "email": "martin@martinfowler.com" }

Response Status: 200 OK Parameters:

{ "name": "Martin Fowler", "email": "martin@martinfowler.com", "id": 11 }
