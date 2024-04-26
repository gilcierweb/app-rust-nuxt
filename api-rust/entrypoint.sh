#!/bin/bash
echo "Starting migrations..."
ls -laH
diesel migration run
echo "End migrations..."