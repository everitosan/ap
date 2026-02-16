# Endpoints spec

apibase: /api/v1

## register & login
**paths**: /register, login 
**payload**
```
{
  "telephone": ""
}
```
**payload validations**
- Should contain numbers
- length must be 10
- only mexican numbers are allowed

**system**  
- should create alphanumeric code of lenght 5
- if phone does not exists at 'users' table, create
- associate the code with the user at 'validation_codes' table
- send that code over phone-notifier

**response**  
- api response ok
- status: 201


## validate code
**path**: /phone-validate
**payload**
```
{
  "code": ""
}
```
**payload validations**
- Should contain numbers
- length must be 10
- only mexican numbers are allowed