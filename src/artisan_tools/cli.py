from artisan_tools.app import App


def run():
    """
    Run the CLI
    """
    app = App()
    return app.cli()
