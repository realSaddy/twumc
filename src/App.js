import React from "react";
import jwt_decode from "jwt-decode";
import { BrowserRouter as Router, Switch, Route, Link } from "react-router-dom";

export default function App() {
  return (
    <Router>
      <div>
        <nav>
          <ul>
            <li>
              <Link to="/">Home</Link>
            </li>
            <li>
              <Link to="/test">Test</Link>
            </li>
          </ul>
        </nav>
      </div>
      <Switch>
        <AuthedRoute path="/test" component={Test} />
        <AuthedRoute path="/" component={Index} />
      </Switch>
    </Router>
  );
}

function AuthedRoute({ component, authed, ...rest }) {
  return (
    <Route
      {...rest}
      render={(_props) => {
        try {
          const t = localStorage.getItem("auth");
          const token = jwt_decode(t);
          console.log(token);
          if (token.exp < Date.now()) {
            throw new Error();
          } else {
            return (
              <Component
                token={token}
                reEvaluateToken={reEvaluateToken}
                {...rest}
              />
            );
          }
        } catch (err) {
          if (localStorage.getItem("auth") !== null)
            localStorage.removeItem("auth");
          return <h1>Login pls!</h1>;
        }
      }}
    />
  );
}

function Test() {
  return <h2>test</h2>;
}

function Index() {
  return <h1>Index</h1>;
}
