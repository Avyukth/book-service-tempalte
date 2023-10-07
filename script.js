import http from "k6/http";
import { check, sleep } from "k6";

export const options = {
  vus: 250,
  duration: "30s",
};

export default function () {
  const res = http.get("http://0.0.0.0:8090/books/");
  check(res, { "status was 200": (r) => r.status == 200 });
  sleep(1);
}
