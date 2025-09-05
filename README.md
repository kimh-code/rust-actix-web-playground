# Rust Acitx-Web Playground
Work in Progress - This project is under active development

## 로드맵
1. GraphQL API 구축
2. REST API 구축
3. 하이브리드 API (GraphQL + REST)
4. React 프론트엔드 연동

## 프로젝트들
- 'graphql' - GraphQL 서버
- 'rest' - RESTAPI 서버
- 'hybrid' - GraphQL+ REST 통합

## 기술 스택
- **Backend**: Rust, Actix-web, async-graphql
- **Frontend**: React, TypeScript

# 프로젝트 다운로드 및 설치
```bash
git clone https://github.com/kimh-code/rust-actix-web-playground.git
cd rust-actix-web-playground
```

## Quick Start
```bash
# 1. DB 시작 
docker start rust-actix-web-db

# 2. 서버 시작
cd backend/graphql
cargo run

# 3. GraphQL Playground 열기
http://localhost:8000/playground
```

## API 예시
```graphql
# 사용자 생성
mutation {
  createUser(input:{
    username:"testuser"
    email:"test@example.com"
    password:"mypassword"
  }){
    id
    username
    email
  }
}

# ID로 사용자 찾기
query{
  user(id:"456439f1-9102-4c1c-a70f-4deb2f492643") {
    id
    username
    email
  }
}

# ID리스트로 사용자 조회
query {
  users(ids: [
    "생성된-ID-1",
    "생성된-ID-2",
    "생성된-ID-3"
  ]) {
    id
    username
    email
  }
}

# 모든 사용자 찾기
query {
  findAll {
    id
    username
    email
  }
}

# 프로필ID, displayName 보기
query {
  user(id: "d9e31784-b9f8-4a42-becf-3218a7fdae05") {
    id
    username
    email
    profileId
    displayName
  }
}
```